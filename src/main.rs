use crate::application::services::user::service::UserService;

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

    let id_provider = implementation::uuid7_id_provider::UserIDProviderUUID7::new();

    let user_service = UserService::new(db_pool, repo, id_provider.clone());

    let id = "062ee66d-bf14-71a6-8000-26806ef51b28"
        .to_string()
        .try_into()?;

    let u = user_service.get_by_id(&id).await?;

    println!("{:?}", u);

    Ok(())
}
