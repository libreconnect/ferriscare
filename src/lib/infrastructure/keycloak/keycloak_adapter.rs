use std::sync::Arc;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::domain::shared::user_identity::{
    UserIdentityError, UserIdentityInfo, UserIdentityProvider,
};

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakResponseCreateToken {
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct KeycloakAdapter {
    client: Arc<Client>,
    keycloak_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
}

impl KeycloakAdapter {
    pub fn new(
        keycloak_url: String,
        realm: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            client: Arc::new(Client::new()),
            keycloak_url,
            realm,
            client_id,
            client_secret,
        }
    }

    pub async fn get_access_token(&self) -> Result<String, UserIdentityError> {
        let uri = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.keycloak_url, self.realm
        );

        let body = json!({
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "grant_type": "client_credentials",
        });

        let response = self
            .client
            .post(uri)
            .form(&body)
            .send()
            .await
            .map_err(|e| UserIdentityError::CreateTokenError(e.to_string()))?;

        let data: KeycloakResponseCreateToken = response
            .json()
            .await
            .map_err(|e| UserIdentityError::CreateTokenError(e.to_string()))?;

        Ok(data.access_token)
    }
}

impl UserIdentityProvider for KeycloakAdapter {
    async fn create_user(&self, data: UserIdentityInfo) -> Result<String, UserIdentityError> {
        let uri = format!("{}/admin/realms/{}/users", self.keycloak_url, self.realm);
        let token = self.get_access_token().await?;

        let body = json!({
            "username": data.username,
            "email": data.email,
            "firstName": data.first_name,
            "lastName": data.last_name,
            "enabled": true,
        });

        let response = self
            .client
            .post(uri)
            .json(&body)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| UserIdentityError::CreateUserError(e.to_string()))?;

        if response.status().is_success() {
            if let Some(location) = response.headers().get("Location") {
                let location_str = location
                    .to_str()
                    .map_err(|_| UserIdentityError::InvalidResponse)?
                    .split("/")
                    .last()
                    .ok_or(UserIdentityError::InvalidResponse)?;

                return Ok(location_str.to_string());
            }
        }

        Err(UserIdentityError::CreateUserError(
            "Error creating user".to_string(),
        ))
    }

    async fn delete_user(&self, _user_id: &str) -> Result<(), UserIdentityError> {
        todo!()
    }
}
