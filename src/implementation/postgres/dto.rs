use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::user::entity::User;
use crate::error::{Error, Result};

#[derive(Debug, FromRow)]
pub struct UserFromRepo {
    id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
}

impl TryFrom<UserFromRepo> for User {
    type Error = Error;

    fn try_from(value: UserFromRepo) -> Result<Self> {
        Ok(Self {
            id: value.id.into(),
            email: value.email.try_into()?,
            first_name: value.first_name,
            last_name: value.last_name,
        })
    }
}
