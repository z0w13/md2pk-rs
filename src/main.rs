use clap::Parser;

use crate::config::{Config, Flags};

mod config;
mod errors;

fn main() {
    let flags = Flags::parse();
    let conf = Config::load(flags).expect("error parsing config");
}
