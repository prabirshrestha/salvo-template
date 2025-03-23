use std::sync::Arc;

use async_trait::async_trait;
use secrecy::ExposeSecret;
use time::OffsetDateTime;
use tracing::{info, warn};
use validator::Validate;

use crate::{
    AppError, AppResult,
    services::user::{CreateUserRequest, CreateUserResponse, UserService},
    utils::datetime::to_surreal_datetime,
};

#[derive(Debug, Clone)]
pub struct SurrealUserService {
    db: Arc<surrealdb::Surreal<surrealdb::engine::any::Any>>,
}

impl SurrealUserService {
    pub fn new(db: Arc<surrealdb::Surreal<surrealdb::engine::any::Any>>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserService for SurrealUserService {
    async fn create_user(&self, request: CreateUserRequest) -> AppResult<CreateUserResponse> {
        request.validate()?;

        let now = to_surreal_datetime(&OffsetDateTime::now_utc());

        let sql = r#"
                CREATE user CONTENT {
                    name: $name,
                    username: $username,
                    email: $email,
                    password: crypto::argon2::generate($raw_password),
                    created_at: $created_at,
                    updated_at: $updated_at
                } RETURN meta::id(id) as id, name, username, email, raw_password, created_at, updated_at
            "#;

        let mut result = self
            .db
            .query(sql)
            .bind(("name", request.name.trim().to_owned()))
            .bind(("username", request.username.trim().to_owned()))
            .bind(("email", request.email.trim().to_owned()))
            .bind(("raw_password", request.password.expose_secret().to_owned()))
            .bind(("created_at", now.clone().to_owned()))
            .bind(("updated_at", now.clone().to_owned()))
            .await?
            .check()?;

        let user: Option<CreateUserResponse> = result.take(0)?;

        match user {
            Some(user) => {
                info!("User created successfully. user_id={}", &user.id);
                Ok(user)
            }
            None => {
                warn!("Failed to create user username={}", &request.username);
                Err(AppError::InternalServerError(
                    "Failed to create user".to_string(),
                ))
            }
        }
    }
}
