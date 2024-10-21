use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Professional {
    pub id: uuid::Uuid,
    pub name: Name,
    pub email: Email,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct ProfessionalRow {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct ProfessionalAddRequestMessage {
    pub professional_id: String,
    pub company_id: String,
}

impl From<&ProfessionalRow> for Professional {
    fn from(professional_row: &ProfessionalRow) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&professional_row.id).unwrap(),
            name: Name::new(professional_row.name.as_str()).unwrap(),
            email: Email::new(professional_row.email.as_str()).unwrap(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProfessionalError {
    #[error("Error creating professional: {0}")]
    CreateError(String),
    #[error("{0}")]
    DuplicateEmail(String),
    #[error("Professional not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Email(String);

#[derive(Clone, Debug, Error)]
#[error("Name cannot be empty")]
pub struct NameEmptyError;

impl Name {
    pub fn new(value: &str) -> Result<Name, NameEmptyError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            Err(NameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Error)]
#[error("Email cannot be empty")]
pub struct EmailEmptyError;

impl Email {
    pub fn new(value: &str) -> Result<Email, EmailEmptyError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            Err(EmailEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
