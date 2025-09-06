use std::path::{Path, PathBuf};

use crate::config::{GroupFieldConfig, MemberFieldConfig};
use eyre::eyre;
use frontmatter_gen::Frontmatter;

#[derive(Debug)]
pub(crate) struct MarkdownMember {
    pub(crate) path: PathBuf,
    pub(crate) id: String,
    pub(crate) uuid: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) pronouns: Vec<String>,
    pub(crate) proxy_tags: Vec<String>,
    pub(crate) private: Option<bool>,
}

impl MarkdownMember {
    pub(crate) fn from_markdown(
        path: &Path,
        frontmatter: &Frontmatter,
        content: &str,
        cfg: &MemberFieldConfig,
    ) -> eyre::Result<Self> {
        let Some(id_val) = frontmatter.get(&cfg.id) else {
            return Err(eyre!("couldn't find id field `{}`", cfg.id));
        };
        let Some(id_str) = id_val.as_str() else {
            return Err(eyre!("couldn't find id field `{}`", cfg.id));
        };

        let uuid = cfg
            .uuid
            .as_ref()
            .and_then(|uuid_field| frontmatter.get(uuid_field))
            .and_then(|uuid_val| uuid_val.as_str())
            .map(|str| str.to_owned());

        let name = cfg
            .name
            .as_ref()
            .and_then(|name_field| frontmatter.get(name_field))
            .and_then(|name_val| name_val.as_str())
            .map(|str| str.to_owned())
            .or_else(|| {
                path.file_stem()
                    .map(|stem| String::from(stem.to_string_lossy()))
            });

        let display_name = cfg
            .display_name
            .as_ref()
            .and_then(|display_name_field| frontmatter.get(display_name_field))
            .and_then(|display_name_val| display_name_val.as_str())
            .map(|str| str.to_owned());

        let private = cfg
            .private
            .as_ref()
            .and_then(|private_field| frontmatter.get(private_field))
            .and_then(|private_val| private_val.as_bool());

        Ok(Self {
            id: id_str.to_owned(),
            uuid,
            name,
            display_name,
            pronouns: Vec::new(),
            proxy_tags: Vec::new(),
            private,
            path: PathBuf::from(path),
        })
    }
}

#[derive(Debug)]
pub(crate) struct MarkdownGroup {
    pub(crate) path: PathBuf,
    pub(crate) id: String,
    pub(crate) uuid: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) private: Option<bool>,
}

impl MarkdownGroup {
    pub(crate) fn from_markdown(
        path: &Path,
        frontmatter: &Frontmatter,
        content: &str,
        cfg: &GroupFieldConfig,
    ) -> eyre::Result<Self> {
        let Some(id_val) = frontmatter.get(&cfg.id) else {
            return Err(eyre!("couldn't find id field `{}`", cfg.id));
        };
        let Some(id_str) = id_val.as_str() else {
            return Err(eyre!("couldn't find id field `{}`", cfg.id));
        };

        let uuid = cfg
            .uuid
            .as_ref()
            .and_then(|uuid_field| frontmatter.get(uuid_field))
            .and_then(|uuid_val| uuid_val.as_str())
            .map(|str| str.to_owned());

        let name = cfg
            .name
            .as_ref()
            .and_then(|name_field| frontmatter.get(name_field))
            .and_then(|name_val| name_val.as_str())
            .map(|str| str.to_owned())
            .or_else(|| {
                path.file_stem()
                    .map(|stem| String::from(stem.to_string_lossy()))
            });

        let display_name = cfg
            .display_name
            .as_ref()
            .and_then(|display_name_field| frontmatter.get(display_name_field))
            .and_then(|display_name_val| display_name_val.as_str())
            .map(|str| str.to_owned());

        let private = cfg
            .private
            .as_ref()
            .and_then(|private_field| frontmatter.get(private_field))
            .and_then(|private_val| private_val.as_bool());

        Ok(Self {
            id: id_str.to_owned(),
            uuid,
            name,
            display_name,
            private,
            path: PathBuf::from(path),
        })
    }
}
