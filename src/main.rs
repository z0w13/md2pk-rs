use clap::Parser;
use color_eyre::eyre::Result;
use eyre::eyre;
use std::fs;

use crate::config::{Command, CommandLine, Config};

mod config;
mod markdown;
mod scan_result;
mod scanner_paths;
mod scanner_tags;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = CommandLine::parse();
    if matches!(&cli.command, Command::Config) {
        if fs::exists(&cli.config)? {
            return Err(eyre!("ERROR: config file {} already exists", cli.config));
        }

        let cfg = Config::default();
        let serialized = toml::to_string_pretty(&cfg)?;
        fs::write(&cli.config, serialized)?;

        return Ok(());
    }

    let conf = Config::load(&cli)?;
    match &cli.command {
        Command::Sync { execute } => {
            let files = match conf.scan_type.as_str() {
                "tags" => scanner_tags::run(conf.tag_scanner),
                "path" => scanner_paths::run(conf.path_scanner),
                _ => Err(eyre!(
                    "Unknown `scan_type` '{}' valid options are `tags` and `path`",
                    conf.scan_type
                )),
            }?;

            println!("{files:?}");

            Ok(())
        }
        Command::Config => {
            // handled above
            unreachable!()
        }
    }
}
