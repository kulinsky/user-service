use uuid::Uuid;

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
    fn provide() -> Result<UserID>;
}
