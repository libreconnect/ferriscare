use std::sync::Arc;

use ferriscare::{
    application::http::{HttpServer, HttpServerConfig},
    domain::professional::services::ProfessionalServiceImpl,
    infrastructure::{db::neo4j::Neo4j, professional::neo4j::professional_repository::Neo4jProfessionalRepository},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database = Neo4j::new("0.0.0.0:7687", "neo4j", "password").await;
    let neo4j = Arc::new(database);

    let server_config = HttpServerConfig { port: "3333" };

    let professional_repository = Neo4jProfessionalRepository::new(Arc::clone(&neo4j));
    let professional_service = ProfessionalServiceImpl::new(professional_repository);

    let http_server = HttpServer::new(server_config, Arc::new(professional_service)).await?;
    http_server.run().await
}
