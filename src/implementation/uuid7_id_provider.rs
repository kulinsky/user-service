use uuid;
use uuid7;

use crate::domain::user::value_objects::{UserID, UserIDProvider};
use crate::error::Result;

#[derive(Clone)]
pub struct UserIDProviderUUID7 {}

impl UserIDProviderUUID7 {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserIDProvider for UserIDProviderUUID7 {
    fn provide(&self) -> Result<UserID> {
        Ok(uuid::Uuid::from_slice(uuid7::uuid7().as_bytes())?.into())
    }
}
