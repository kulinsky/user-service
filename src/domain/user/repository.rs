use async_trait::async_trait;

use crate::error::Result;

use super::entity::User;
use super::value_objects::UserID;

#[async_trait]
pub trait Repository {
    async fn get_by_id(&self, id: &UserID) -> Result<User>;
}
