use super::value_objects::{Email, UserID};

#[derive(Debug, Clone)]
pub struct User {
    id: UserID,
    email: Email,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(id: UserID, email: Email, first_name: String, last_name: String) -> Self {
        User {
            id,
            email,
            first_name,
            last_name,
        }
    }

    pub fn id(&self) -> UserID {
        self.id.clone()
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}
