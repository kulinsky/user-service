use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;

use super::entity::User;

#[async_trait]
pub trait Repository {
    async fn get_by_id(&self, id: &Uuid) -> Result<User>;
}
