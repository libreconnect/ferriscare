use std::sync::Arc;

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

    async fn handle_create_relationship(
        &self,
        message: ProfessionalAddRequestMessage,
    ) -> Result<(), ProfessionalError> {
        info!("Handling create relationship message: {:?}", message);

        Ok(())
    }
}
