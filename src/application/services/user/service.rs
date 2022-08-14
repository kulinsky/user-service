use crate::domain::user::entity::User;
use crate::domain::user::value_objects::UserID;
use crate::error::{Error, Result};
use crate::infrastructure::database::DB;

use super::repository::Repository;

pub struct UserService<R>
where
    R: Repository,
{
    repository: R,
    queryer: DB,
}

impl<R> UserService<R>
where
    R: Repository,
{
    pub fn new(queryer: DB, repository: R) -> Self {
        Self {
            queryer,
            repository,
        }
    }

    pub async fn get_by_id(&self, id: &UserID) -> Result<User> {
        self.repository.get_by_id(&self.queryer, id).await
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let mut tx = self.queryer.begin().await?;

        self.repository.create(&mut tx, user).await?;

        tx.commit().await.map_err(Error::from)
    }
}
