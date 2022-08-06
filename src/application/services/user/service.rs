use crate::domain::user::value_objects::UserID;
use crate::domain::user::{entity::User, repository::Repository};
use crate::error::Result;

pub struct UserService<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_by_id(&self, id: &UserID) -> Result<User> {
        self.repository.get_by_id(id).await
    }
}
