use std::str::FromStr;

use crate::domain::user::{repository::Repository, value_objects::UserID};

mod error;
mod implementation;
mod infrastructure;

mod domain;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = infrastructure::config::Config::new_from_env()?;

    let db_pool = infrastructure::database::create_pool(&config).await?;

    infrastructure::database::ping(&db_pool).await?;

    let repo = implementation::postgres::repository::UserPGRepository::new(db_pool);

    let test_id = uuid::Uuid::from_str("062ee66d-bf14-71a6-8000-26806ef51b28").unwrap();

    let test_user_id = UserID::from(test_id);

    let u = repo.get_by_id(&test_user_id).await?;

    let id = domain::user::value_objects::UserID {
        value: uuid::Uuid::new_v4(),
    };

    println!("{:?}", u);

    Ok(())
}
