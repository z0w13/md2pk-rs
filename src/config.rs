use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(version)]
pub(crate) struct Flags {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    pub(crate) config: String,

    /// Only print changes
    #[arg(short, long, default_value_t = false)]
    pub(crate) quiet: bool,

    /// Actually perform changes
    #[arg(short, long, default_value_t = false)]
    pub(crate) execute: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NameConfig {
    display_name_pronouns: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    scan_type: ScanConfig,
    token: String,
    name: NameConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ScanConfig {
    Tag(TagScanConfig),
    Path(PathScanConfig),
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TagScanConfig {
    root_dir: String,
    member_tags: Option<Vec<String>>,
    group_tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PathScanConfig {
    recursive: bool,
    member_dir: Option<String>,
    group_dir: Option<String>,
}

impl Config {
    #[expect(clippy::result_large_err, reason = "only used once")]
    pub(crate) fn load(flags: Flags) -> eyre::Result<Config, figment::Error> {
        Figment::new()
            .merge(Serialized::defaults(&flags))
            .merge(Toml::file(flags.config))
            .merge(Env::prefixed("MD2PK_"))
            .extract()
    }
}
