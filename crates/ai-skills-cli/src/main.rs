#![forbid(unsafe_code)]

use std::{env, path::PathBuf, process::ExitCode};

mod config;

const HELP: &str =
    "ai-skills\n\nUsage:\n  ai-skills [--config <path>] config <check|show>\n  ai-skills --help";

fn main() -> ExitCode {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    if arguments.first().is_some_and(|value| value == "--help") || arguments.is_empty() {
        println!("{HELP}");
        return ExitCode::SUCCESS;
    }

    let (config_path, command_index) = if arguments.first().is_some_and(|value| value == "--config")
    {
        match arguments.get(1) {
            Some(path) => (PathBuf::from(path), 2),
            None => return fail("--config requires a path"),
        }
    } else {
        (PathBuf::from("config.toml"), 0)
    };

    match arguments
        .get(command_index)
        .and_then(|value| value.to_str())
    {
        Some("config") => match arguments
            .get(command_index + 1)
            .and_then(|value| value.to_str())
        {
            Some("check") => match config::Config::load(&config_path) {
                Ok(_) => {
                    println!("configuration is valid");
                    ExitCode::SUCCESS
                }
                Err(error) => fail(&error),
            },
            Some("show") => match config::Config::load(&config_path) {
                Ok(config) => {
                    println!("{config:#?}");
                    ExitCode::SUCCESS
                }
                Err(error) => fail(&error),
            },
            _ => fail("usage: ai-skills [--config <path>] config <check|show>"),
        },
        _ => fail("usage: ai-skills [--config <path>] config <check|show>"),
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::from(2)
}
