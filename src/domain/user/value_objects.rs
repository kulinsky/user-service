use std::str::FromStr;

use uuid::Uuid;

use validator::Validate;

use crate::error::{Error, Result};

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

impl TryFrom<String> for UserID {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Uuid::from_str(&value)?.into())
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

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Email::new(value)
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
