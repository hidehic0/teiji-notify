mod load_config;
mod parse_time;
mod types;

use crate::parse_time::parse_time;
use chrono::Timelike;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"),version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    GetTasks {
        #[arg(short, long = "config")]
        config_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        SubCommands::GetTasks { config_path } => {
            let tasks = match load_config::load_config(config_path.to_string()) {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            };

            for task in tasks {
                let time = match parse_time(task.time) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e);
                        process::exit(1);
                    }
                };

                println!("{} {}:{}", task.title, time.hour(), time.minute());
            }
        }
    }
}
