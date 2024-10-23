use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::application::ports::messaging_ports::MessagingPort;

use super::{
    models::{Professional, ProfessionalAddRequestMessage, ProfessionalError},
    ports::{ProfessionalRepository, ProfessionalService},
};

#[derive(Debug, Clone)]
pub struct ProfessionalServiceImpl<P, M>
where
    P: ProfessionalRepository,
    M: MessagingPort,
{
    professional_repository: P,
    #[allow(dead_code)]
    messaging: Arc<M>,
}

impl<P, M> ProfessionalServiceImpl<P, M>
where
    P: ProfessionalRepository,
    M: MessagingPort,
{
    pub fn new(professional_repository: P, messaging: Arc<M>) -> Self {
        Self {
            professional_repository,
            messaging,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct Message {
    message: String,
    professional_id: String,
    company_id: String,
}

impl<P, M> ProfessionalService for ProfessionalServiceImpl<P, M>
where
    P: ProfessionalRepository,
    M: MessagingPort,
{
    async fn create(&self, name: &str, email: &str) -> Result<Professional, ProfessionalError> {
        self.professional_repository.create(name, email).await
    }

    async fn find_by_id(&self, id: String) -> Result<Professional, ProfessionalError> {
        self.professional_repository.find_by_id(id).await
    }

    // @TODO: remove the unwrap in serde_json::to_string and handle the error
    // @TOOD: move the struct Message in domain
    async fn handle_create_relationship(
        &self,
        message: ProfessionalAddRequestMessage,
    ) -> Result<(), ProfessionalError> {
        info!("Handling create relationship message: {:?}", message);

        let id = message.professional_id;
        let professional = self.find_by_id(id.clone()).await;

        match professional {
            Ok(professional) => {
                info!("Professional found: {:?}", professional);

                let msg = Message {
                    message: format!("Professional {} found", &id),
                    professional_id: professional.id.to_string(),
                    company_id: message.company_id,
                };

                self.messaging
                    .publish_message(
                        "professional.add.validated".to_string(),
                        serde_json::to_string(&msg).unwrap(),
                    )
                    .await
                    .map_err(|_| {
                        ProfessionalError::NotFound(format!("Professional {} not found", &id))
                    })?;
            }
            Err(e) => {
                info!("Error finding professional: {:?}", e);

                let msg = Message {
                    message: format!("Professional {} not found", &id),
                    professional_id: id.clone(),
                    company_id: message.company_id,
                };

                let msg_str = serde_json::to_string(&msg).unwrap();

                self.messaging
                    .publish_message("professional.add.failure".to_string(), msg_str)
                    .await
                    .map_err(|_| {
                        ProfessionalError::NotFound(format!("Professional {} not found", &id))
                    })?;
            }
        }

        Ok(())
    }
}
