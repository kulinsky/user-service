use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::error::Result;

pub enum StoreTx<'a> {
    PGTX(Transaction<'a, Postgres>),
}

#[async_trait]
pub trait Store<'a> {
    async fn begin(&self) -> Result<Option<StoreTx>>;

    async fn commit(maybe_tx: Option<StoreTx<'_>>) -> Result<()>;

    async fn rollback(maybe_tx: Option<StoreTx<'_>>) -> Result<()>;
}
