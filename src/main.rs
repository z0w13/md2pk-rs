use clap::Parser;
use color_eyre::eyre::Result;

use crate::config::{Config, Flags};

mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    let flags = Flags::parse();
    let conf = Config::load(flags)?;

    Ok(())
}
