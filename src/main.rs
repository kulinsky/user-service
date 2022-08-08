use std::str::FromStr;

use crate::{
    application::services::user::service::UserService,
    domain::user::{
        entity::User,
        repository::Repository,
        value_objects::{Email, UserID, UserIDProvider},
    },
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

    let test_id = uuid::Uuid::from_str("01827df4-cedd-7532-b110-03d430d3556f").unwrap();

    let test_user_id = UserID::from(test_id);

    let id_provider = implementation::uuid7_id_provider::UserIDProviderUUID7::new();

    let test_u = User {
        email: Email::new("test_email@email.org".to_string()).unwrap(),
        id: id_provider.provide().unwrap(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    // let _ = repo.create(&test_u).await?;

    // println!("{:?}", test_u);

    let user_service = UserService::new(repo, id_provider);

    let u = user_service.get_by_id(&test_user_id).await?;

    println!("{:?}", u);

    Ok(())
}
