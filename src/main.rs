use clap::Parser;
use reqwest;
use std::{env, fmt::Display, fs, process};
use tokio;

#[derive(clap::Parser, Debug, Default)]
struct ReadCommand;

#[derive(clap::Parser, Debug)]
struct Config;

/// A CLI so I can learn Rust
#[derive(clap::Parser, Debug)]
enum SubCommand {
    /// Install a new Node.js version
    #[clap(name = "read")]
    Read(ReadCommand),
}

/// A fast and simple Node.js manager.
#[derive(clap::Parser, Debug)]
#[clap(name = "cli")]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug)]
enum CommandType {
    Help,
    Version,
    ReadFile,
    ReadDirectory,
    HealthCommand,
}

impl ReadCommand {
    fn apply() -> () {
        println!("Read");
    }
    fn call() -> () {
        println!("Read");
    }
}

trait CommandX {
    fn run(&self) -> ();
    fn new(args: &Vec<String>) -> Self;
}

impl CommandX for ReadCommand {
    fn new(_args: &Vec<String>) -> Self {
        ReadCommand {}
    }
    fn run(&self) -> () {
        println!("This is the help menu")
    }
}

async fn health_check() -> Result<String, reqwest::Error> {
    let g = reqwest::get("https://partybox.im/api/health")
        .await?
        .text()
        .await?;

    return Ok(g);
}

impl CommandType {
    fn parse(action_str: &String) -> Result<CommandType, String> {
        match action_str.to_lowercase().as_str() {
            "help" => Ok(CommandType::Help),
            "version" => Ok(CommandType::Version),
            "read" => Ok(CommandType::ReadFile),
            "dir" => Ok(CommandType::ReadDirectory),
            "health" => Ok(CommandType::HealthCommand),
            _ => Err(String::from("Action string is not a valid command type")),
        }
    }
}

impl Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandType::Help => "help",
                CommandType::ReadFile => "read",
                CommandType::Version => "version",
                CommandType::ReadDirectory => "dir",
                CommandType::HealthCommand => "health",
            }
        )
    }
}

impl SubCommand {
    fn call(&self) {
        println!("Called")
    }
}

struct Command {
    t: CommandType,
    args: Vec<String>,
}

impl Command {
    async fn execute(&self) -> Result<String, String> {
        return match self.t {
            CommandType::Help => Ok(String::from("Here is the help menu")),
            CommandType::HealthCommand => {
                let body = match health_check().await {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };

                return Ok(body);
            }
            CommandType::ReadDirectory => {
                let path = match self.args.get(0) {
                    Some(x) => x,
                    None => return Err(String::from("Missing directory path")),
                };

                let mut output = String::new();

                let files = match fs::read_dir(path) {
                    Err(e) => return Err(e.to_string()),
                    Ok(x) => x,
                };

                for (i, f) in files.enumerate() {
                    let name = f.unwrap().file_name();
                    let name_str = name.to_str().unwrap();

                    if i > 0 {
                        output.push('\n');
                    }

                    output.push_str(name_str);
                }

                return Ok(output);
            }
            CommandType::ReadFile => {
                let path = match self.args.get(0) {
                    Some(x) => x,
                    None => return Err(String::from("Missing file path")),
                };

                let data =
                    fs::read_to_string(path).map_or_else(|x| Err(x.to_string()), |e| Ok(e))?;

                return Ok(data);
            }
            CommandType::Version => Ok(String::from("v1.0")),
        };
    }

    fn new(args: Vec<String>) -> Result<Command, String> {
        let first_arg = match args.get(0) {
            None => return Err(String::from("Action string is missing.")),
            Some(x) => x,
        };

        let action = CommandType::parse(&first_arg)?;

        return Ok(Command {
            t: action,
            args: args.into_iter().skip(1).collect(),
        });
    }
}

// ./cli help
// ./cli version
// ./cli read file.txt
// ./cli dir

fn main() {
    let args = Cli::parse();

    args.subcmd.call()

    // for _ in 0..args.count {
    // println!("Hello {}!", args.name)
    // }
    // let args = env::args().skip(1).collect();

    // let c = match Command::new(args) {
    //     Ok(x) => x,
    //     Err(e) => {
    //         eprintln!("ERROR: {}", e);
    //         process::exit(1);
    //     }
    // };

    // let output = match c.execute().await {
    //     Ok(x) => x,
    //     Err(e) => {
    //         eprintln!("ERROR: {}", e);
    //         process::exit(1);
    //     }
    // };

    // println!("{}", output);
}
