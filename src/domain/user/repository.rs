use async_trait::async_trait;

use crate::error::Result;
use crate::infrastructure::database::Queryer;

use super::entity::User;
use super::value_objects::UserID;

#[async_trait]
pub trait Repository {
    async fn get_by_id<'c, C: Queryer<'c>>(&self, db: C, id: &UserID) -> Result<User>;

    async fn create<'c, C: Queryer<'c>>(&self, db: C, user: &User) -> Result<()>;
}
