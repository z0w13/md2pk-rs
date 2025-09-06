use std::collections::HashSet;

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
    /// list local groups and members
    List,
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
    pub(crate) scan_type: String,
    pub(crate) fields: FieldConfig,
    pub(crate) scanner: ScanConfig,
    pub(crate) token: String,
    pub(crate) name: NameConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: String::from("YOUR_PK_TOKEN"),
            scan_type: String::from("path"),
            fields: FieldConfig::default(),
            scanner: ScanConfig::default(),
            name: NameConfig::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct ScanConfig {
    pub(crate) tags: TagScanConfig,
    pub(crate) path: PathScanConfig,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct FieldConfig {
    pub(crate) member: MemberFieldConfig,
    pub(crate) group: GroupFieldConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MemberFieldConfig {
    pub(crate) id: String,
    pub(crate) uuid: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) pronouns: Option<String>,
    pub(crate) proxy_tags: Option<String>,
    pub(crate) private: Option<String>,
}

impl Default for MemberFieldConfig {
    fn default() -> Self {
        MemberFieldConfig {
            id: String::from("id"),
            uuid: None,
            name: Some(String::from("name")),
            display_name: Some(String::from("display_name")),
            pronouns: Some(String::from("pronouns")),
            proxy_tags: Some(String::from("proxy_tags")),
            private: Some(String::from("private")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GroupFieldConfig {
    pub(crate) id: String,
    pub(crate) uuid: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) private: Option<String>,
}

impl Default for GroupFieldConfig {
    fn default() -> Self {
        GroupFieldConfig {
            id: String::from("id"),
            uuid: None,
            name: Some(String::from("name")),
            display_name: Some(String::from("display_name")),
            private: Some(String::from("private")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TagScanConfig {
    pub(crate) root_dir: String,
    pub(crate) member_tags: Option<HashSet<String>>,
    pub(crate) group_tags: Option<HashSet<String>>,
}

impl Default for TagScanConfig {
    fn default() -> Self {
        Self {
            root_dir: String::from("~/notes/system"),
            member_tags: Some(HashSet::from([
                String::from("#plurality"),
                String::from("#system/member"),
            ])),
            group_tags: Some(HashSet::from([
                String::from("#plurality"),
                String::from("#system/group"),
            ])),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PathScanConfig {
    pub(crate) recursive: bool,
    pub(crate) member_dir: Option<String>,
    pub(crate) group_dir: Option<String>,
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
    pub(crate) fn load(flags: &CommandLine) -> eyre::Result<Config, figment::Error> {
        Figment::new()
            .merge(Serialized::defaults(&flags))
            .merge(Toml::file(&flags.config))
            .merge(Env::prefixed("MD2PK_"))
            .extract()
    }
}
