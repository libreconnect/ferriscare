use std::sync::Arc;

use crate::domain::{
    patient::models::PatientCreate,
    shared::user_identity::{UserIdentityInfo, UserIdentityProvider},
};

use super::{
    models::{Patient, PatientDto, PatientError},
    ports::{PatientRepository, PatientService},
};

#[derive(Debug, Clone)]
pub struct PatientServiceImpl<P, I>
where
    P: PatientRepository,
    I: UserIdentityProvider,
{
    patient_repository: P,
    identity_provider: Arc<I>,
}

impl<P, I> PatientServiceImpl<P, I>
where
    P: PatientRepository,
    I: UserIdentityProvider,
{
    pub fn new(patient_repository: P, identity_provider: Arc<I>) -> Self {
        Self {
            patient_repository,
            identity_provider,
        }
    }
}

impl<P, I> PatientService for PatientServiceImpl<P, I>
where
    P: PatientRepository,
    I: UserIdentityProvider,
{
    async fn create(&self, dto: PatientDto) -> Result<Patient, PatientError> {
        let user_identity_info = UserIdentityInfo {
            username: dto.username.clone(),
            email: dto.email.clone(),
            first_name: dto.first_name.clone(),
            last_name: dto.last_name.clone(),
        };

        let oidc_ic = self
            .identity_provider
            .create_user(user_identity_info)
            .await
            .map_err(|e| PatientError::CreateError(e.to_string()))?;

        let data = PatientCreate {
            username: dto.username,
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            oidc_id: oidc_ic,
        };

        self.patient_repository
            .create(data)
            .await
            .map_err(|e| PatientError::CreateError(e.to_string()))
    }
}
