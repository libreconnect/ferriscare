use std::sync::Arc;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::professional::{
    models::{EmailEmptyError, NameEmptyError},
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateProfessionalResponseData {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateProfessionalValidator {}

pub async fn create_professional<S: ProfessionalService>(
    Extension(professional_service): Extension<Arc<S>>,
    Json(body): Json<CreateProfessionalValidator>,
) -> Result<ApiSuccess<CreateProfessionalResponseData>, ApiError> {
    todo!()
}
