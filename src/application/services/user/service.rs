use crate::domain::user::value_objects::{UserID, UserIDProvider};
use crate::domain::user::{entity::User, repository::Repository};
use crate::error::Result;
use crate::infrastructure::database::DB;

pub struct UserService<R, P>
where
    R: Repository,
    P: UserIDProvider,
{
    pub repository: R,
    pub id_provider: P,
    pub queryer: DB,
}

impl<R, P> UserService<R, P>
where
    R: Repository,
    P: UserIDProvider,
{
    pub fn new(queryer: DB, repository: R, id_provider: P) -> Self {
        Self {
            queryer,
            repository,
            id_provider,
        }
    }

    pub async fn get_by_id(&self, id: &UserID) -> Result<User> {
        self.repository.get_by_id(&self.queryer, id).await
    }
}
