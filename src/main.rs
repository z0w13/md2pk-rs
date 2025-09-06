use clap::Parser;
use color_eyre::eyre::Result;
use eyre::eyre;
use std::fs;
use tabled::{builder::Builder, settings::Style};

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

            let mut group_builder = Builder::new();
            let total_groups = files.groups.len();
            group_builder.push_record(["ID", "UUID", "Name", "Display Name", "Prv"]);
            for group in files.groups {
                group_builder.push_record([
                    group.id,
                    group.uuid.unwrap_or_default(),
                    group.name.unwrap_or_default(),
                    group.display_name.unwrap_or_default(),
                    String::from(
                        group
                            .private
                            .map_or_else(|| "❔", |v| if v { "✔️" } else { "❌" }),
                    ),
                ]);
            }
            group_builder.push_record(["", "", "", "", "", &format!("Total: {total_groups}")]);
            let mut group_table = group_builder.build();
            group_table.with(Style::modern_rounded());
            println!("{group_table}");

            println!();

            let mut member_builder = Builder::new();
            let total_members = files.members.len();
            member_builder.push_record([
                "ID",
                "UUID",
                "Name",
                "Display Name",
                "Prv",
                "Pronouns",
                "Proxy Tags",
            ]);
            for member in files.members {
                member_builder.push_record([
                    member.id,
                    member.uuid.unwrap_or_default(),
                    member.name.unwrap_or_default(),
                    member.display_name.unwrap_or_default(),
                    String::from(
                        member
                            .private
                            .map_or_else(|| "❔", |v| if v { "✔️" } else { "❌" }),
                    ),
                    member.pronouns.join("\n"),
                    member.proxy_tags.join("\n"),
                ]);
            }
            member_builder.push_record(["", "", "", "", "", &format!("Total: {total_members}")]);
            let mut member_table = member_builder.build();
            member_table.with(Style::modern_rounded());
            println!("{member_table}");

            Ok(())
        }
        Command::Config => {
            // handled above
            unreachable!()
        }
    }
}
