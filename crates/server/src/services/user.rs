use anyhow::Result;
use async_trait::async_trait;

use crate::{entities::user::UserEntity, models::user::SignupRequest};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<UserEntity>>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
    async fn validate_user(&self, username: &str, password: &str) -> Result<()>;
    async fn signup(&self, signup_request: &SignupRequest) -> Result<String>;
    async fn lock_user(&self, user_id: &str) -> Result<()>;
}
