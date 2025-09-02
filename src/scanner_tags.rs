use frontmatter_gen::Frontmatter;
use std::collections::HashSet;

use crate::{config::TagScanConfig, markdown, scan_result::ScanResult};

fn parse_tags(frontmatter: Frontmatter) -> Option<HashSet<String>> {
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

pub(crate) fn run(cfg: TagScanConfig) -> eyre::Result<ScanResult> {
    let mut members = Vec::new();
    let mut groups = Vec::new();

    for md_path in markdown::walker(&cfg.root_dir, true)
        .map(|entry| entry.path().to_string_lossy().into_owned())
    {
        let file_content = match std::fs::read_to_string(&md_path) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {md_path}: {err}");
                continue;
            }
        };

        let (frontmatter, content) = match frontmatter_gen::extract(&file_content) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {md_path}: {err}");
                continue;
            }
        };

        let Some(tags) = parse_tags(frontmatter) else {
            println!("DEBUG {md_path}: no tags, skipping");
            continue;
        };

        if let Some(member_tags) = &cfg.member_tags
            && member_tags.is_subset(&tags)
        {
            members.push(md_path.clone())
        }

        if let Some(group_tags) = &cfg.group_tags
            && group_tags.is_subset(&tags)
        {
            groups.push(md_path.clone())
        }
    }

    Ok(ScanResult { members, groups })
}
