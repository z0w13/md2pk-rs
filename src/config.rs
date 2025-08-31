use clap::{Parser, Subcommand};
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(version)]
pub(crate) struct CommandLine {
    /// Path to config file
    #[arg(short, long, global = true, default_value = "config.toml")]
    pub(crate) config: String,

    /// Only print changes
    #[arg(short, long, global = true, default_value_t = false)]
    pub(crate) quiet: bool,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub(crate) enum Command {
    /// sync system from markdown to pluralkit
    Sync {
        /// Actually perform changes
        #[arg(short, long, default_value_t = false)]
        execute: bool,
    },
    /// generate config file
    Config,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NameConfig {
    display_name_pronouns: bool,
}

#[expect(
    clippy::derivable_impls,
    reason = "explicit is better, expect more flags in the future"
)]
impl Default for NameConfig {
    fn default() -> Self {
        Self {
            display_name_pronouns: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    scan_type: ScanConfig,
    token: String,
    name: NameConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: String::from("YOUR_PK_TOKEN"),
            scan_type: ScanConfig::default(),
            name: NameConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ScanConfig {
    Tag(TagScanConfig),
    Path(PathScanConfig),
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self::Path(PathScanConfig::default())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TagScanConfig {
    root_dir: String,
    member_tags: Option<Vec<String>>,
    group_tags: Option<Vec<String>>,
}

impl Default for TagScanConfig {
    fn default() -> Self {
        Self {
            root_dir: String::from("~/notes/system"),
            member_tags: Some(vec![
                String::from("#plurality"),
                String::from("#system/member"),
            ]),
            group_tags: Some(vec![
                String::from("#plurality"),
                String::from("#system/group"),
            ]),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PathScanConfig {
    recursive: bool,
    member_dir: Option<String>,
    group_dir: Option<String>,
}

impl Default for PathScanConfig {
    fn default() -> Self {
        Self {
            recursive: false,
            member_dir: Some(String::from("~/notes/system/members")),
            group_dir: Some(String::from("~/notes/system/groups")),
        }
    }
}

impl Config {
    #[expect(clippy::result_large_err, reason = "only used once")]
    pub(crate) fn load(flags: CommandLine) -> eyre::Result<Config, figment::Error> {
        Figment::new()
            .merge(Serialized::defaults(&flags))
            .merge(Toml::file(flags.config))
            .merge(Env::prefixed("MD2PK_"))
            .extract()
    }
}
