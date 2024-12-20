use std::sync::Arc;

use clap::Parser;
use ferriscare::{
    application::{
        http::{HttpServer, HttpServerConfig},
        ports::{
            messaging_ports::MessagingPort, messaging_subscriber_port::MessagingSubscriberPort,
        },
    },
    domain::{
        patient::services::PatientServiceImpl,
        professional::{
            models::ProfessionalAddRequestMessage,
            ports::{ProfessionalRepository, ProfessionalService},
            services::ProfessionalServiceImpl,
        },
    },
    env::Env,
    infrastructure::{
        db::neo4j::Neo4j, keycloak::keycloak_adapter::KeycloakAdapter,
        messaging::nats::NatsMessaging, patient::neo4j::patient_repository::Neo4jPatientRepository,
        professional::neo4j::professional_repository::Neo4jProfessionalRepository,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let database = Neo4j::new(
        &env.database_url,
        &env.database_user,
        &env.database_password,
    )
    .await;
    let neo4j = Arc::new(database);

    let messaging = Arc::new(NatsMessaging::new(&env.nats_url).await?);

    let server_config = HttpServerConfig::new(env.port.clone());

    let user_identity_provider = Arc::new(KeycloakAdapter::new(
        env.keycloak_url.clone(),
        env.keycloak_realm.clone(),
        env.keycloak_client_id.clone(),
        env.keycloak_client_secret.clone(),
    ));

    let patient_repository = Neo4jPatientRepository::new(Arc::clone(&neo4j)).await?;
    let patient_service =
        PatientServiceImpl::new(patient_repository, Arc::clone(&user_identity_provider));

    let professional_repository = Neo4jProfessionalRepository::new(Arc::clone(&neo4j)).await;
    let professional_service =
        ProfessionalServiceImpl::new(professional_repository, Arc::clone(&messaging));

    let professional_service = Arc::new(professional_service);
    let patient_service = Arc::new(patient_service);

    start_subscriptions(Arc::clone(&messaging), Arc::clone(&professional_service)).await;

    let http_server = HttpServer::new(
        server_config,
        Arc::clone(&professional_service),
        Arc::clone(&patient_service),
        Arc::clone(&env),
    )
    .await?;
    http_server.run().await
}

async fn start_subscriptions<P, M>(
    messaging: Arc<NatsMessaging>,
    professional_service: Arc<ProfessionalServiceImpl<P, M>>,
) where
    P: ProfessionalRepository,
    M: MessagingPort,
{
    let professional_service_cloned = Arc::clone(&professional_service);
    let messaing_clone = Arc::clone(&messaging);

    tokio::spawn(async move {
        let result = messaing_clone
            .subscribe(
                "company.professional.add.requested",
                move |e: ProfessionalAddRequestMessage| {
                    let t = Arc::clone(&professional_service_cloned);
                    async move {
                        let professional_service = Arc::clone(&t);
                        let _ = professional_service.handle_create_relationship(e).await;
                        Ok(())
                    }
                },
            )
            .await;

        if let Err(e) = result {
            eprintln!("Error during subscription: {:?}", e);
        }
    });
}
