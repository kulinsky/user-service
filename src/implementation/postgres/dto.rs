use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::user::entity::User;
use crate::domain::user::value_objects::{Email, UserID};

#[derive(Debug, FromRow)]
pub struct UserFromRepo {
    id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
}

impl From<UserFromRepo> for User {
    fn from(dto: UserFromRepo) -> Self {
        Self {
            id: UserID::from(dto.id),
            email: Email { value: dto.email },
            first_name: dto.first_name,
            last_name: dto.last_name,
        }
    }
}
