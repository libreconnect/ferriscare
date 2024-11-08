use std::future::Future;

use super::models::{Patient, PatientCreate, PatientDto, PatientError};

pub trait PatientRepository: Clone + Send + Sync + 'static {
    fn create(
        &self,
        data: PatientCreate,
    ) -> impl Future<Output = Result<Patient, PatientError>> + Send;
}

pub trait PatientService: Clone + Send + Sync + 'static {
    fn create(&self, dto: PatientDto)
        -> impl Future<Output = Result<Patient, PatientError>> + Send;
}
