use super::{
    models::{Professional, ProfessionalError},
    ports::{ProfessionalRepository, ProfessionalService},
};

#[derive(Debug, Clone)]
pub struct ProfessionalServiceImpl<P>
where
    P: ProfessionalRepository,
{
    professional_repository: P,
}

impl<P> ProfessionalServiceImpl<P>
where
    P: ProfessionalRepository,
{
    pub fn new(professional_repository: P) -> Self {
        Self {
            professional_repository,
        }
    }
}

impl<P> ProfessionalService for ProfessionalServiceImpl<P>
where
    P: ProfessionalRepository,
{
    async fn create(&self) -> Result<Professional, ProfessionalError> {
        todo!()
    }
}
