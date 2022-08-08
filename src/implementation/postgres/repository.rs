use async_trait::async_trait;

use sqlx::pool::Pool;
use sqlx::postgres::Postgres;

use crate::domain::user::value_objects::UserID;
use crate::error::Result;

use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository;

use super::dto::UserFromRepo;

const SQL_GET_USER_BY_ID: &str = "select * from users where id = $1";
const SQL_INSERT_USER: &str =
    "insert into users (id, email, first_name, last_name) values ($1, $2, $3, $4)";

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
        let result: UserFromRepo = sqlx::query_as(SQL_GET_USER_BY_ID)
            .bind(&user_id.value())
            .fetch_one(&self.pool)
            .await?;

        Ok(User::from(result))
    }

    async fn create(&self, user: &User) -> Result<()> {
        sqlx::query(SQL_INSERT_USER)
            .bind(&user.id.value)
            .bind(&user.email.value)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
