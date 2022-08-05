use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, PgRow, Postgres};

use crate::error::{Error, ErrorKind, Result};

use super::config::Config;

pub async fn create_pool(config: &Config) -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(config.database.max_conn)
        .connect(&config.database.get_connect_string())
        .await
        .map_err(Error::from)
}

pub async fn ping(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query("select 1").fetch_one(pool).await?;

    Ok(())
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error {
                kind: ErrorKind::NotFound,
                message: String::from("no rows found"),
            },
            _ => Error {
                kind: ErrorKind::Repository("Postgres".to_string()),
                message: err.to_string(),
            },
        }
    }
}
