use super::command::Command;
use crate::config::Config;
use async_trait::async_trait;
use std::fs;

#[derive(clap::Parser, Debug, Default)]
pub struct ReadCommand {
    file_name: String,
}

#[async_trait]
impl Command for ReadCommand {
    async fn call(&self, _config: &Config) -> () {
        let data = fs::read_to_string(&self.file_name).unwrap();

        println!("{}", data);
    }
}
