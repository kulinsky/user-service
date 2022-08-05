use crate::error::Result;

pub struct UserID {
    value: i64,
}

pub trait UserIDProvider {
    pub fn provide() -> Result<UserID>;
}