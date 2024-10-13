use std::sync::Arc;

use ferriscare::{
    application::http::{HttpServer, HttpServerConfig},
    infrastructure::db::neo4j::Neo4j,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database = Neo4j::new("0.0.0.0:7687", "neo4j", "password").await;
    let neo4j = Arc::new(database);

    let server_config = HttpServerConfig { port: "3333" };

    // let professional_repository = Neo4jProfessionalRepository::new(Arc::clone(&neo4j));

    // let _ = professional_repository
    //     .create("Nathael", "nathael@bonnal.cloud")
    //     .await;
    // let _ = professional_repository
    //     .create("Martin", "martin@matin.io")
    //     .await;

    let http_server = HttpServer::new(server_config).await?;
    http_server.run().await
}
