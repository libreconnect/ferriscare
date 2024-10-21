use std::future::Future;

use super::models::{Professional, ProfessionalAddRequestMessage, ProfessionalError};

pub trait ProfessionalRepository: Clone + Send + Sync + 'static {
    fn create(
        &self,
        name: &str,
        email: &str,
    ) -> impl Future<Output = Result<Professional, ProfessionalError>> + Send;
    fn find_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<Professional, ProfessionalError>> + Send;
}

pub trait ProfessionalService: Clone + Send + Sync + 'static {
    fn create(
        &self,
        name: &str,
        email: &str,
    ) -> impl Future<Output = Result<Professional, ProfessionalError>> + Send;
    fn find_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<Professional, ProfessionalError>> + Send;
    fn handle_create_relationship(
        &self,
        message: ProfessionalAddRequestMessage,
    ) -> impl Future<Output = Result<(), ProfessionalError>> + Send;
}
