use std::sync::Arc;

use neo4rs::query;

use crate::{
    domain::patient::{
        models::{Patient, PatientCreate, PatientError},
        ports::PatientRepository,
    },
    infrastructure::db::neo4j::Neo4j,
};

#[derive(Clone)]
pub struct Neo4jPatientRepository {
    neo4j: Arc<Neo4j>,
}

impl Neo4jPatientRepository {
    pub async fn new(neo4j: Arc<Neo4j>) -> Result<Neo4jPatientRepository, anyhow::Error> {
        let patient_repository = Neo4jPatientRepository { neo4j };

        patient_repository.initialize_constraints().await?;

        Ok(patient_repository)
    }

    pub async fn initialize_constraints(&self) -> Result<(), anyhow::Error> {
        let create_constraint_query = query("
            CREATE CONSTRAINT unique_patient_username IF NOT EXISTS  FOR (p:Patient) REQUIRE p.username IS UNIQUE;
        ");

        let constraint_email_query = query("
            CREATE CONSTRAINT unique_patient_email IF NOT EXISTS  FOR (p:Patient) REQUIRE p.email IS UNIQUE;
        ");

        self.neo4j.get_graph().run(constraint_email_query).await?;
        self.neo4j.get_graph().run(create_constraint_query).await?;

        Ok(())
    }
}

impl PatientRepository for Neo4jPatientRepository {
    async fn create(&self, data: PatientCreate) -> Result<Patient, PatientError> {
        let patient = Patient {
            id: uuid::Uuid::new_v4(),
            username: data.username.clone(),
            first_name: data.first_name.clone(),
            last_name: data.last_name.clone(),
            email: data.email.clone(),
            oidc_id: data.oidc_id.clone(),
        };

        let query = query("CREATE (p:Patient { username: $username, first_name: $first_name, last_name: $last_name, email: $email, oidc_id: $oidc_id, id: $id })");

        self.neo4j
            .get_graph()
            .run(query)
            .await
            .map_err(|e| PatientError::CreateError(e.to_string()))?;

        Ok(patient)
    }
}
