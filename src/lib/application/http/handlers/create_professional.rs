use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::professional::{
    models::{EmailEmptyError, NameEmptyError, Professional, ProfessionalError},
    ports::ProfessionalService,
};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, Error)]
enum ParseCreateProfessionalError {
    #[error(transparent)]
    NameEmpty(#[from] NameEmptyError),
    #[error(transparent)]
    EmailEmpty(#[from] EmailEmptyError),
}

impl From<ParseCreateProfessionalError> for ApiError {
    fn from(e: ParseCreateProfessionalError) -> Self {
        let message = match e {
            ParseCreateProfessionalError::EmailEmpty(e) => e.to_string(),
            ParseCreateProfessionalError::NameEmpty(e) => e.to_string(),
        };

        Self::UnProcessableEntity(message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateProfessionalResponseData {
    id: String,
    name: String,
    email: String,
}

impl From<&Professional> for CreateProfessionalResponseData {
    fn from(professional: &Professional) -> Self {
        CreateProfessionalResponseData {
            id: professional.id.to_string(),
            name: professional.name.as_str().to_string(),
            email: professional.email.as_str().to_string(),
        }
    }
}

impl From<ProfessionalError> for ApiError {
    fn from(e: ProfessionalError) -> Self {
        match e {
            ProfessionalError::CreateError(e) => Self::InternalServerError(e),
            ProfessionalError::DuplicateEmail(e) => Self::InternalServerError(e),
            ProfessionalError::NotFound(e) => Self::NotFound(e),
            ProfessionalError::DatabaseError(e) => Self::InternalServerError(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateProfessionalValidator {
    name: String,
    email: String,
}

pub async fn create_professional<P: ProfessionalService>(
    Extension(professional_service): Extension<Arc<P>>,
    Json(body): Json<CreateProfessionalValidator>,
) -> Result<ApiSuccess<CreateProfessionalResponseData>, ApiError> {
    let name = body.name;
    let email = body.email;

    professional_service
        .create(&name, &email)
        .await
        .map_err(ApiError::from)
        .map(|ref professional| ApiSuccess::new(StatusCode::CREATED, professional.into()))
}
