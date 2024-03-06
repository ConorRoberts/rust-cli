use async_trait::async_trait;
use clap::Parser;
use reqwest;
use std::fs;
use tokio;

#[derive(clap::Parser, Debug, Default)]
struct ReadCommand {
    file_name: String,
}

#[derive(Debug, clap::Parser)]
struct ReadDirectoryCommand {
    path: String,
}

#[derive(clap::Parser, Debug)]
struct Config;

/// A CLI so I can learn Rust
#[derive(clap::Parser, Debug)]
enum SubCommand {
    /// Install a new Node.js version
    #[clap(name = "read")]
    Read(ReadCommand),

    #[clap(name = "dir")]
    ReadDir(ReadDirectoryCommand),

    #[clap(name = "health")]
    Health(HealthCommand),
}

/// A fast and simple Node.js manager.
#[derive(clap::Parser, Debug)]
#[clap(name = "cli")]
struct Cli {
    #[clap(flatten)]
    config: Config,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[async_trait]
trait Command {
    async fn call(&self, config: &Config) -> ();
}

#[async_trait]
impl Command for ReadCommand {
    async fn call(&self, _config: &Config) -> () {
        let data = fs::read_to_string(&self.file_name).unwrap();

        println!("{}", data);
    }
}

async fn health_check() -> Result<String, reqwest::Error> {
    let g = reqwest::get("https://partybox.im/api/health")
        .await?
        .text()
        .await?;

    return Ok(g);
}

impl SubCommand {
    async fn call(&self, config: Config) {
        match self {
            SubCommand::Read(cmd) => cmd.call(&config).await,
            SubCommand::Health(cmd) => cmd.call(&config).await,
            SubCommand::ReadDir(cmd) => cmd.call(&config).await,
        }
    }
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

#[derive(Debug, clap::Parser)]
struct HealthCommand;
#[async_trait]
impl Command for HealthCommand {
    async fn call(&self, _config: &Config) -> () {
        let body = health_check().await.unwrap();

        println!("{}", body);
    }
}

// ./cli help
// ./cli version
// ./cli read file.txt
// ./cli dir

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    args.subcmd.call(args.config).await;
}
