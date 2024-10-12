use std::sync::Arc;

use anyhow::Ok;
use ferriscare::{domain::professional::ports::ProfessionalRepository, infrastructure::{db::neo4j::Neo4j, professional::neo4j::professional_repository::Neo4jProfessionalRepository}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    
    let database = Neo4j::new("0.0.0.0:7687", "neo4j", "password").await;
    let neo4j = Arc::new(database);

    let professional_repository = Neo4jProfessionalRepository::new(Arc::clone(&neo4j));

    let _ = professional_repository.create("Nathael", "nathael@bonnal.cloud").await;
    let _ = professional_repository.create("Martin", "martin@matin.io").await;

    Ok(())
}
