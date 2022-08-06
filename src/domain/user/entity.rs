use super::value_objects::{Email, UserID};

#[derive(Debug)]
pub struct User {
    pub id: UserID,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
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
}
