use std::str::FromStr;

use crate::domain::user::repository::Repository;

mod application;
mod domain;
mod error;
mod implementation;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = infrastructure::config::Config::new_from_env()?;

    let db_pool = infrastructure::database::connect(&config.database).await?;

    infrastructure::database::ping(&db_pool).await?;

    let repo = implementation::postgres::repository::UserRepositoryPG::new();

    let test_user_id = uuid::Uuid::from_str("01827df4-cedd-7532-b110-03d430d3556f")?.into();

    let id_provider = implementation::uuid7_id_provider::UserIDProviderUUID7::new();

    let u = repo.get_by_id(&db_pool, &test_user_id).await?;

    println!("{:?}", u);

    let mut tx = db_pool.begin().await?;

    let u2 = repo.get_by_id(&mut tx, &test_user_id).await?;

    println!("{:?}", u2);

    Ok(())
}
