use std::future::Future;

use super::models::{Professional, ProfessionalError};

pub trait ProfessionalRepository: Clone + Send + Sync + 'static {
  fn create(&self) -> impl Future<Output = Result<Professional, ProfessionalError>> + Send;
}

pub trait ProfessionalService: Clone + Send + Sync + 'static {}