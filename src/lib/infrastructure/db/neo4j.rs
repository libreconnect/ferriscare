use neo4rs::*;
use std::sync::Arc;

pub struct Neo4j {
    pub graph: Arc<Graph>,
}

impl Neo4j {
    pub async fn new(uri: &str, user: &str, password: &str) -> Neo4j {
        let graph = Graph::new(uri, user, password).await.unwrap();

        Neo4j {
            graph: Arc::new(graph),
        }
    }

    pub fn get_graph(&self) -> Arc<Graph> {
        Arc::clone(&self.graph)
    }
}
