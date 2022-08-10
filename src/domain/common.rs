use async_trait::async_trait;

#[async_trait]
pub trait Transactional {
    async fn with_tx(&self) -> Self;
}
