use std::sync::Arc;

use clap::Parser;
use ferriscare::{
    application::http::{HttpServer, HttpServerConfig},
    domain::professional::services::ProfessionalServiceImpl,
    env::Env,
    infrastructure::{
        db::neo4j::Neo4j, messaging::nats::NatsMessaging,
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

    let _messaging = Arc::new(NatsMessaging::new(&env.nats_url).await?);

    // tokio::spawn(async move {
    //     messaging
    //         .subscribre("test", |e: Message| {
    //             println!("{:?}", e);
    //             Ok(())
    //         })
    //         .await
    //         .unwrap();
    // });

    let server_config = HttpServerConfig::new(env.port.clone());

    let professional_repository = Neo4jProfessionalRepository::new(Arc::clone(&neo4j)).await;
    let professional_service = ProfessionalServiceImpl::new(professional_repository);

    let http_server = HttpServer::new(server_config, Arc::new(professional_service)).await?;
    http_server.run().await
}
