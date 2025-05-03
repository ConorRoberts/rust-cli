use super::command::Command;
use crate::config::Config;
use async_trait::async_trait;
use std::fs;

#[derive(Debug, clap::Parser)]
pub struct ReadDirectoryCommand {
    path: String,
}

#[async_trait]
impl Command for ReadDirectoryCommand {
    async fn call(&self, _config: &Config) -> () {
        let mut output = String::new();

        let files = fs::read_dir(&self.path).unwrap();
        for (i, f) in files.enumerate() {
            let name = f.unwrap().file_name();
            let name_str = name.to_str().unwrap();

            if i > 0 {
                output.push('\n');
            }

            output.push_str(name_str);
        }

        println!("{}", output)
    }
}
