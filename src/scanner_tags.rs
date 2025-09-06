use frontmatter_gen::Frontmatter;
use std::collections::HashSet;

use crate::{
    config::{FieldConfig, TagScanConfig},
    markdown,
    markdown_objects::{MarkdownGroup, MarkdownMember},
    scan_result::ScanResult,
};

fn parse_tags(frontmatter: &Frontmatter) -> Option<HashSet<String>> {
    let Some(tags_val) = frontmatter.get("tags") else {
        println!("INFO  : no tags, skipping");
        return None;
    };

    let Some(tags) = tags_val.as_array() else {
        println!("WARN  : couldn't parse tags '{tags_val:?}' as array, skipping");
        return None;
    };

    let mut tag_set = HashSet::new();
    for val in tags {
        let Some(tag_str) = val.as_str() else {
            println!("WARN  : couldn't parse tag '{val:?}' as string, skipping");
            continue;
        };

        println!("INFO  {tag_str}");
        tag_set.insert(tag_str.to_owned());
    }

    Some(tag_set)
}

pub(crate) fn run(cfg: &TagScanConfig, field_cfg: &FieldConfig) -> eyre::Result<ScanResult> {
    let mut members = Vec::new();
    let mut groups = Vec::new();

    for md_entry in markdown::walker(&cfg.root_dir, true) {
        let file_content = match std::fs::read_to_string(md_entry.path()) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", md_entry.path().display());
                continue;
            }
        };

        let (frontmatter, content) = match frontmatter_gen::extract(&file_content) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", md_entry.path().display());
                continue;
            }
        };

        let Some(tags) = parse_tags(&frontmatter) else {
            println!("DEBUG {}: no tags, skipping", md_entry.path().display());
            continue;
        };

        if let Some(member_tags) = &cfg.member_tags
            && member_tags.is_subset(&tags)
        {
            match MarkdownMember::from_markdown(
                md_entry.path(),
                &frontmatter,
                content,
                &field_cfg.member,
            ) {
                Err(err) => {
                    println!("ERROR {}: {err}", md_entry.path().display());
                    continue;
                }
                Ok(member) => members.push(member),
            }
        }

        if let Some(group_tags) = &cfg.group_tags
            && group_tags.is_subset(&tags)
        {
            match MarkdownGroup::from_markdown(
                md_entry.path(),
                &frontmatter,
                content,
                &field_cfg.group,
            ) {
                Err(err) => {
                    println!("ERROR {}: {err}", md_entry.path().display());
                    continue;
                }
                Ok(group) => groups.push(group),
            }
        }
    }

    Ok(ScanResult { members, groups })
}
