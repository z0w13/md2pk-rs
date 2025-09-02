use crate::{config::PathScanConfig, markdown, scan_result::ScanResult};

fn scan_members(path: String, recursive: bool) -> eyre::Result<Vec<String>> {
    Ok(markdown::walker(&path, recursive)
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect())
}

fn scan_groups(path: String, recursive: bool) -> eyre::Result<Vec<String>> {
    Ok(markdown::walker(&path, recursive)
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect())
}

pub(crate) fn run(cfg: PathScanConfig) -> eyre::Result<ScanResult> {
    let members = if let Some(member_dir) = cfg.member_dir {
        scan_members(member_dir, cfg.recursive)?
    } else {
        Vec::new()
    };

    let groups = if let Some(group_dir) = cfg.group_dir {
        scan_groups(group_dir, cfg.recursive)?
    } else {
        Vec::new()
    };

    Ok(ScanResult { members, groups })
}
