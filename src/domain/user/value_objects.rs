use uuid::Uuid;

use validator::Validate;

use crate::error::Result;

#[derive(Debug)]
pub struct UserID {
    pub value: Uuid,
}

impl UserID {
    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for UserID {
    fn from(id: Uuid) -> Self {
        Self { value: id }
    }
}

impl PartialEq for UserID {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub trait UserIDProvider {
    fn provide(&self) -> Result<UserID>;
}

#[derive(Debug, Validate)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn new(s: String) -> Result<Email> {
        let email = Self { value: s };

        email.validate()?;

        Ok(email)
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
