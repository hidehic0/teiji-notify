mod load_config;
mod parse_time;
mod types;

use crate::parse_time::parse_time;
use chrono::{Local, Timelike};
use clap::{Parser, Subcommand};
use notify_rust::Notification;
use std::{process, thread, time};

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
    Deamon {
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
        SubCommands::Deamon { config_path } => {
            let tasks = match load_config::load_config(config_path.to_string()) {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            };

            let mut now = Local::now().time();
            thread::sleep(time::Duration::from_secs(60 - now.second() as u64));

            loop {
                now = Local::now().time();

                for task in &tasks {
                    let time = match parse_time(task.time.clone()) {
                        Ok(t) => t,
                        Err(e) => {
                            println!("{}", e);
                            process::exit(1);
                        }
                    };

                    if time.hour() == now.hour() && time.minute() == now.minute() {
                        if task.detail.is_none() {
                            Notification::new()
                                .summary(&task.title)
                                .body("")
                                .appname("teiji-notify")
                                .show()
                                .unwrap();
                        } else {
                            let detail: &str = task.detail.as_ref().unwrap();
                            Notification::new()
                                .summary(&task.title)
                                .appname("teiji-notify")
                                .body(detail)
                                .show()
                                .unwrap();
                        }
                    }
                }

                now = Local::now().time();
                thread::sleep(time::Duration::from_secs(60 - now.second() as u64));
            }
        }
    }
}
