use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}
