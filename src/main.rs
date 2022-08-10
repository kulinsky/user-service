use std::str::FromStr;

use crate::{application::services::user::service::UserService, store::Store};

mod application;
mod error;
mod implementation;
mod infrastructure;
mod store;

mod domain;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = infrastructure::config::Config::new_from_env()?;

    let db_pool = infrastructure::database::create_pool(&config).await?;

    infrastructure::database::ping(&db_pool).await?;

    let repo = implementation::postgres::repository::UserPGRepository::new(db_pool);

    let test_user_id = uuid::Uuid::from_str("01827df4-cedd-7532-b110-03d430d3556f")?.into();

    let id_provider = implementation::uuid7_id_provider::UserIDProviderUUID7::new();

    let user_service = UserService::new(repo, id_provider);

    let u = user_service.get_by_id(&test_user_id).await?;

    println!("{:?}", u);

    let tx = user_service.repository.begin().await?;

    user_service.repository.rollback(tx).await?;

    Ok(())
}
