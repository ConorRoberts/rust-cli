use crate::config::Config;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn call(&self, config: &Config) -> ();
}
