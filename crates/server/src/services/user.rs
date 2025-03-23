use async_trait::async_trait;
use salvo::oapi::{ToSchema, schema};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;

use crate::AppResult;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[salvo(schema(example = "johndoe"))]
    pub username: String,
    #[salvo(schema(example = "John Doe"))]
    pub name: String,
    #[salvo(schema(example = "john.doe@example.com"))]
    #[validate(email)]
    pub email: String,
    #[salvo(schema(value_type=String, example = "changeme"))]
    pub password: SecretString,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserResponse {
    #[salvo(schema(example = "lvjjq45c5objddsbcnj4"))]
    pub id: String,
    #[salvo(schema(example = "johndoe"))]
    pub username: String,
    #[salvo(schema(example = "johndoe"))]
    pub email: String,
    #[salvo(schema(example = "John Doe"))]
    pub name: String,
    #[salvo(schema(example = "2025-03-23T20:15:08.564688Z"))]
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[salvo(schema(example = "2025-03-23T20:15:08.564688Z"))]
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> AppResult<CreateUserResponse>;
}
