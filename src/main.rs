mod commands;
mod config;
use crate::commands::command::Command;
use crate::commands::health::HealthCommand;
use crate::commands::read::ReadCommand;
use crate::commands::read_dir::ReadDirectoryCommand;
use crate::config::Config;
use clap::Parser;
use commands::write::WriteCommand;
use tokio;

/// A CLI so I can learn Rust
#[derive(clap::Parser)]
enum SubCommand {
    /// Read a file
    #[clap(name = "read")]
    Read(ReadCommand),

    /// List the contents of a directory (ls)
    #[clap(name = "dir")]
    ReadDir(ReadDirectoryCommand),

    /// Perform a health check on https://partybox.im
    #[clap(name = "health")]
    Health(HealthCommand),

    /// Write some data
    #[clap(name = "write")]
    Write(WriteCommand),
}

#[derive(clap::Parser)]
#[clap(name = "cli")]
struct Cli {
    #[clap(flatten)]
    config: Config,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl SubCommand {
    async fn call(&self, config: Config) {
        match self {
            SubCommand::Read(cmd) => cmd.call(&config).await,
            SubCommand::Health(cmd) => cmd.call(&config).await,
            SubCommand::ReadDir(cmd) => cmd.call(&config).await,
            SubCommand::Write(cmd) => cmd.call(&config).await,
        }
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
