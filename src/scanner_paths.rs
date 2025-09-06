use crate::{
    config::{FieldConfig, GroupFieldConfig, MemberFieldConfig, PathScanConfig},
    markdown,
    markdown_objects::{MarkdownGroup, MarkdownMember},
    scan_result::ScanResult,
};

fn scan_members(
    path: &str,
    recursive: bool,
    field_cfg: &MemberFieldConfig,
) -> eyre::Result<Vec<MarkdownMember>> {
    let mut members = Vec::new();
    for entry in markdown::walker(&path, recursive) {
        let file_content = match std::fs::read_to_string(&entry.path()) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
        };

        let (frontmatter, content) = match frontmatter_gen::extract(&file_content) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
        };

        match MarkdownMember::from_markdown(entry.path(), &frontmatter, content, &field_cfg) {
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
            Ok(member) => members.push(member),
        }
    }

    Ok(members)
}

fn scan_groups(
    path: &str,
    recursive: bool,
    field_cfg: &GroupFieldConfig,
) -> eyre::Result<Vec<MarkdownGroup>> {
    let mut groups = Vec::new();
    for entry in markdown::walker(&path, recursive) {
        let file_content = match std::fs::read_to_string(&entry.path()) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
        };

        let (frontmatter, content) = match frontmatter_gen::extract(&file_content) {
            Ok(val) => val,
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
        };

        match MarkdownGroup::from_markdown(entry.path(), &frontmatter, content, &field_cfg) {
            Err(err) => {
                println!("ERROR {}: {err}", entry.path().display());
                continue;
            }
            Ok(group) => groups.push(group),
        }
    }

    Ok(groups)
}

pub(crate) fn run(cfg: &PathScanConfig, field_cfg: &FieldConfig) -> eyre::Result<ScanResult> {
    let members = if let Some(member_dir) = &cfg.member_dir {
        scan_members(member_dir, cfg.recursive, &field_cfg.member)?
    } else {
        Vec::new()
    };

    let groups = if let Some(group_dir) = &cfg.group_dir {
        scan_groups(group_dir, cfg.recursive, &field_cfg.group)?
    } else {
        Vec::new()
    };

    Ok(ScanResult { members, groups })
}
