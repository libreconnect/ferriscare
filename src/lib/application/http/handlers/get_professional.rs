use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Serialize;

use crate::domain::professional::ports::ProfessionalService;

use super::{ApiError, ApiSuccess};

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProfessionalResponse {
    id: String,
    name: String,
    email: String,
}

pub async fn get_professional<P: ProfessionalService>(
    Extension(professional_service): Extension<Arc<P>>,
    Path(professional_id): Path<String>,
) -> Result<ApiSuccess<ProfessionalResponse>, ApiError> {
    professional_service
        .find_by_id(professional_id)
        .await
        .map_err(ApiError::from)
        .map(|ref professional| {
            ApiSuccess::new(
                StatusCode::OK,
                ProfessionalResponse {
                    id: professional.id.to_string(),
                    name: professional.name.as_str().to_string(),
                    email: professional.email.as_str().to_string(),
                },
            )
        })
}
