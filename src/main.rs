use clap::Parser;
use color_eyre::eyre::Result;
use eyre::eyre;
use std::fs;

use crate::config::{Command, CommandLine, Config};

mod config;
mod markdown;
mod markdown_objects;
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
                "tags" => scanner_tags::run(conf.tag_scanner, conf.fields),
                "path" => scanner_paths::run(conf.path_scanner, conf.fields),
                _ => Err(eyre!(
                    "Unknown `scan_type` '{}' valid options are `tags` and `path`",
                    conf.scan_type
                )),
            }?;

            println!("|={:=^102}=|", "GROUPS");
            println!(
                "| {:6} | {:40} | {:40} | {:7} |",
                "ID", "Name", "Display Name", "Private"
            );
            println!("|-{:-<6}-+-{:-<40}-+-{:-<40}-+-{:-<7}-|", "", "", "", "");
            for group in files.groups {
                println!(
                    "| {:6} | {:40} | {:40} | {:7} |",
                    group.id,
                    group.name.unwrap_or_default(),
                    group.display_name.unwrap_or_default(),
                    group
                        .private
                        .map_or_else(|| "N/A", |v| if v { "Y" } else { "N" }),
                );
            }
            println!("|={:=^102}=|", "");

            println!();

            println!("|={:=^102}=|", "MEMBERS");
            println!(
                "| {:6} | {:40} | {:40} | {:7} |",
                "ID", "Name", "Display Name", "Private"
            );
            println!("|-{:-<6}-+-{:-<40}-+-{:-<40}-+-{:-<7}-|", "", "", "", "");
            for member in files.members {
                println!(
                    "| {:6} | {:40} | {:40} | {:7} |",
                    member.id,
                    member.name.unwrap_or_default(),
                    member.display_name.unwrap_or_default(),
                    member
                        .private
                        .map_or_else(|| "N/A", |v| if v { "Y" } else { "N" }),
                );
            }
            println!("|={:=^102}=|", "");

            Ok(())
        }
        Command::Config => {
            // handled above
            unreachable!()
        }
    }
}
