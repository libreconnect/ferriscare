use std::sync::Arc;

use neo4rs::query;

use crate::{domain::professional::{models::{Email, Name, Professional, ProfessionalError}, ports::ProfessionalRepository}, infrastructure::db::neo4j::Neo4j};

#[derive(Clone)]
pub struct Neo4jProfessionalRepository {
  neo4j: Arc<Neo4j>
}

impl Neo4jProfessionalRepository {
  pub fn new(neo4j: Arc<Neo4j>) -> Neo4jProfessionalRepository {
    Neo4jProfessionalRepository {
      neo4j
    }
  }
}


impl ProfessionalRepository for Neo4jProfessionalRepository {
  async fn create(&self, name: &str, email: &str) -> Result<Professional, ProfessionalError> {
    let professional = Professional {
      id: uuid::Uuid::new_v4(),
      name: Name::new(name).unwrap(),
      email: Email::new(email).unwrap()
    };

    let result = self.neo4j.get_graph().run(
      query("CREATE (p:Professional { name: $name, email: $email })")
        .param("name", professional.name.as_str())
        .param("email", professional.email.as_str())
    ).await;

    match result {
      Ok(_) => {
        Ok(professional)
      }
      Err(_) => {
        Err(ProfessionalError::CreateError("Error creating professional".to_string()))
      }
    }   
  }

  async fn find_by_id(
          &self,
          id: String,
      ) -> Result<Professional, ProfessionalError> {
    let _result = self.neo4j.get_graph().execute(
      query("MATCH (p:Professional) WHERE p.id = $id RETURN p")
        .param("id", id)
    ).await.unwrap();



    Ok(Professional {
      id: uuid::Uuid::new_v4(),
      name: Name::new("John Doe").unwrap(),
      email: Email::new("").unwrap()
    })
  }
}