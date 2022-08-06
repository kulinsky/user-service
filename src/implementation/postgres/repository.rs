use async_trait::async_trait;

use sqlx::pool::Pool;
use sqlx::postgres::Postgres;

use crate::domain::user::value_objects::UserID;
use crate::error::Result;

use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository;

use super::dto::UserFromRepo;

pub struct UserPGRepository {
    pool: Pool<Postgres>,
}

impl UserPGRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for UserPGRepository {
    async fn get_by_id(&self, user_id: &UserID) -> Result<User> {
        let result: UserFromRepo = sqlx::query_as("select * from users where id = $1")
            .bind(&user_id.value())
            .fetch_one(&self.pool)
            .await?;

        Ok(User::from(result))
    }
}
