use std::time::Duration;

use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::{Executor, Transaction};

use super::config;
use crate::error::{Error, ErrorKind, Result};

pub type DB = Pool<Postgres>;
pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}

impl<'c> Queryer<'c> for &Pool<Postgres> {}
impl<'c> Queryer<'c> for &'c mut Transaction<'_, Postgres> {}

pub trait Persister<'c>: Executor<'c, Database = sqlx::Postgres> {}

impl<'c> Persister<'c> for &Pool<Postgres> {}
impl<'c> Persister<'c> for &'c mut Transaction<'_, Postgres> {}

pub async fn connect(database: &config::DbConfig) -> Result<DB> {
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&database.get_connect_string())
        .await
        .map_err(|err| err.into())
}

pub async fn ping(pool: &DB) -> Result<()> {
    const QUERY: &str = "select 1";

    sqlx::query(QUERY).fetch_one(pool).await?;

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
