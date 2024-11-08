use std::future::Future;

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct UserIdentityInfo {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, Error)]
pub enum UserIdentityError {
    #[error("Error creating user: {0}")]
    CreateTokenError(String),

    #[error("Error creating user: {0}")]
    CreateUserError(String),

    #[error("Invalid response")]
    InvalidResponse,
}

pub trait UserIdentityProvider: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        data: UserIdentityInfo,
    ) -> impl Future<Output = Result<String, UserIdentityError>> + Send;

    fn delete_user(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<(), UserIdentityError>> + Send;
}
