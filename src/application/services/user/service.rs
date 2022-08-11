use crate::domain::user::value_objects::{UserID, UserIDProvider};
use crate::domain::user::{entity::User, repository::Repository};
use crate::error::{Error, Result};
use crate::infrastructure::database::DB;

pub struct UserService<R, P>
where
    R: Repository,
    P: UserIDProvider,
{
    repository: R,
    id_provider: P,
    queryer: DB,
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

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let mut tx = self.queryer.begin().await?;

        self.repository.create(&mut tx, user).await?;

        tx.commit().await.map_err(Error::from)
    }
}
