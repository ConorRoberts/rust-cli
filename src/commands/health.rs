use super::command::Command;
use crate::config::Config;
use async_trait::async_trait;
use reqwest;

async fn health_check() -> Result<String, reqwest::Error> {
    let g = reqwest::get("https://partybox.im/api/health")
        .await?
        .text()
        .await?;

    return Ok(g);
}

#[derive(Debug, clap::Parser)]
pub struct HealthCommand;

#[async_trait]
impl Command for HealthCommand {
    async fn call(&self, _config: &Config) -> () {
        let body = health_check().await.unwrap();

        println!("{}", body);
    }
}
