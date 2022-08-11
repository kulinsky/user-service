mod application;
mod domain;
mod error;
mod implementation;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = infrastructure::config::Config::new_from_env()?;

    let db_pool = infrastructure::database::connect(&config.database).await?;

    infrastructure::database::ping(&db_pool).await?;

    let server = presentation::http_server::server::Server::new(config.server.port, db_pool);

    server.run().await;

    Ok(())
}
