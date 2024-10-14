use std::sync::Arc;

use neo4rs::query;

use crate::{
    domain::professional::{
        models::{Email, Name, Professional, ProfessionalError},
        ports::ProfessionalRepository,
    },
    infrastructure::db::neo4j::Neo4j,
};

#[derive(Clone)]
pub struct Neo4jProfessionalRepository {
    neo4j: Arc<Neo4j>,
}

impl Neo4jProfessionalRepository {
    pub async fn new(neo4j: Arc<Neo4j>) -> Neo4jProfessionalRepository {
        let professional_repository = Neo4jProfessionalRepository { neo4j };

        let _ = professional_repository.initialize_constraints().await;

        professional_repository
    }

    pub async fn initialize_constraints(&self) {
        let create_constraint_query = query("
            CREATE CONSTRAINT unique_professional_email IF NOT EXISTS  FOR (p:Professional) REQUIRE p.email IS UNIQUE;
        ");

        let t = self.neo4j.get_graph().run(create_constraint_query).await;

        match t {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

impl ProfessionalRepository for Neo4jProfessionalRepository {
    async fn create(&self, name: &str, email: &str) -> Result<Professional, ProfessionalError> {
        let professional = Professional {
            id: uuid::Uuid::new_v4(),
            name: Name::new(name).unwrap(),
            email: Email::new(email).unwrap(),
        };

        let result = self
            .neo4j
            .get_graph()
            .run(
                query("CREATE (p:Professional { name: $name, email: $email })")
                    .param("name", professional.name.as_str())
                    .param("email", professional.email.as_str()),
            )
            .await;

        match result {
            Ok(_) => Ok(professional),
            Err(e) => {
                println!("{:?}", e);
                match e {
                    neo4rs::Error::Neo4j(neo4j_error) => {
                        println!("{:?}", neo4j_error.code());
                        let code = neo4j_error.code();
                        if code == "Neo.ClientError.Schema.ConstraintValidationFailed" {
                            return Err(ProfessionalError::DuplicateEmail(
                                "Email already exists".to_string(),
                            ));
                        }
                        //Err(ProfessionalError::CreateError("".to_string()))
                        Err(ProfessionalError::CreateError(
                            "error creating professional in neo4j database".to_string(),
                        ))
                    }
                    _ => Err(ProfessionalError::CreateError(format!("{:?}", e))),
                }
            }
        }
    }

    async fn find_by_id(&self, id: String) -> Result<Professional, ProfessionalError> {
        let _result = self
            .neo4j
            .get_graph()
            .execute(query("MATCH (p:Professional) WHERE p.id = $id RETURN p").param("id", id))
            .await
            .unwrap();

        Ok(Professional {
            id: uuid::Uuid::new_v4(),
            name: Name::new("John Doe").unwrap(),
            email: Email::new("").unwrap(),
        })
    }
}
