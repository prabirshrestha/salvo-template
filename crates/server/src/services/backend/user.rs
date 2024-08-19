use crate::{entities::user::UserEntity, models::user::SignupRequest, services::user::UserService};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Any, Pool};

#[derive(Debug, Clone)]
pub struct SqlUserService {
    // db: Pool<Any>,
}

impl SqlUserService {
    pub fn new(_db: Pool<Any>) -> Self {
        Self {}
    }
}

#[async_trait]
impl UserService for SqlUserService {
    async fn get_user_by_id(&self, _user_id: &str) -> Result<Option<UserEntity>> {
        todo!()
    }

    async fn get_user_by_username(&self, _username: &str) -> Result<Option<UserEntity>> {
        todo!()
    }

    async fn validate_user(&self, _username: &str, _password: &str) -> Result<()> {
        todo!()
    }

    async fn signup(&self, _signup_request: &SignupRequest) -> Result<String> {
        todo!()
    }

    async fn lock_user(&self, _user_id: &str) -> Result<()> {
        todo!()
    }
}
