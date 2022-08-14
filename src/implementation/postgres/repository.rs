use async_trait::async_trait;

use crate::application::services::user::repository::Repository;
use crate::domain::user::entity::User;
use crate::domain::user::value_objects::UserID;
use crate::error::Result;
use crate::infrastructure::database::Queryer;

use super::dto::UserFromRepo;

pub struct UserRepositoryPG {}

impl UserRepositoryPG {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository for UserRepositoryPG {
    async fn get_by_id<'c, C: Queryer<'c>>(&self, db: C, user_id: &UserID) -> Result<User> {
        const QUERY: &str = "select * from users where id = $1";

        let result: UserFromRepo = sqlx::query_as(QUERY)
            .bind(&user_id.value())
            .fetch_one(db)
            .await?;

        Ok(result.try_into()?)
    }

    async fn create<'c, C: Queryer<'c>>(&self, db: C, user: &User) -> Result<()> {
        const QUERY: &str =
            "insert into users (id, email, first_name, last_name) values ($1, $2, $3, $4)";

        sqlx::query(QUERY)
            .bind(&user.id().value())
            .bind(&user.email().value())
            .bind(&user.first_name())
            .bind(&user.last_name())
            .execute(db)
            .await?;

        Ok(())
    }
}
