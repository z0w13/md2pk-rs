use crate::{config::TagScanConfig, markdown, scan_result::ScanResult};

pub(crate) fn run(cfg: TagScanConfig) -> eyre::Result<ScanResult> {
    let members = Vec::new();
    let groups = Vec::new();

    for md_path in markdown::walker(&cfg.root_dir, true)
        .map(|entry| entry.path().to_string_lossy().into_owned())
    {
        todo!();
    }

    Ok(ScanResult { members, groups })
}
