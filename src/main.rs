use std::str::FromStr;

use crate::{
    application::services::user::service::UserService, domain::user::value_objects::UserID,
};

mod application;
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

    let id_provider = implementation::uuid7_id_provider::UserIDProviderUUID7::new();

    let user_service = UserService::new(repo, id_provider);

    let u = user_service.get_by_id(&test_user_id).await?;

    println!("{:?}", u);

    Ok(())
}
