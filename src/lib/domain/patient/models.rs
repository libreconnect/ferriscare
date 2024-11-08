use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Patient {
    pub id: uuid::Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub oidc_id: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct PatientCreate {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub oidc_id: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct PatientDto {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Error)]
pub enum PatientError {
    #[error("Error creating patient: {0}")]
    CreateError(String),
    #[error("Patient not found: {0}")]
    NotFound(String),
}
