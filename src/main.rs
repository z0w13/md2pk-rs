use clap::Parser;
use color_eyre::eyre::Result;

use crate::config::{CommandLine, Config};

mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = CommandLine::parse();
    let conf = Config::load(cli)?;

    Ok(())
}
