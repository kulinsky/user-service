use super::value_objects::UserID;

#[derive(Debug)]
pub struct User {
    pub id: UserID,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(id: UserID, email: String, first_name: String, last_name: String) -> Self {
        User {
            id,
            email,
            first_name,
            last_name,
        }
    }
}
