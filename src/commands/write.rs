use std::fs;

use super::command::Command;
use crate::config::Config;
use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize)]
pub struct WriteCommand {
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Something {
    value: String,
}

#[async_trait]
impl Command for WriteCommand {
    async fn call(&self, _config: &Config) -> () {
        println!("Write");

        let s = Something {
            value: self.value.clone(),
        };

        let serialized = serde_json::to_string(&s).unwrap();
        let encoded: Vec<u8> = bincode::serialize(&s).unwrap();
        fs::write("./something", encoded).unwrap();
        fs::write("./something2", serialized).unwrap();

        let some_bytes = fs::read("./something").unwrap();

        // Take an array of bytes and deserializxe
        let decoded: Something = bincode::deserialize(&some_bytes).unwrap();

        println!("Something: {}", decoded.value);

        return ();
    }
}
