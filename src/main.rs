use clap::Parser;
use color_eyre::eyre::Result;
use eyre::eyre;
use std::fs;

use crate::config::{Command, CommandLine, Config};

mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = CommandLine::parse();
    match &cli.command {
        Command::Config => {
            if fs::exists(&cli.config)? {
                return Err(eyre!("ERROR: config file {} already exists", cli.config));
            }

            let cfg = Config::default();
            let serialized = toml::to_string_pretty(&cfg)?;
            fs::write(&cli.config, serialized)?;
        }
        Command::Sync { execute } => {
            todo!()
        }
    }
    let conf = Config::load(cli)?;

    Ok(())
}
