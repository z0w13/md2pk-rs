use crate::markdown_objects::{MarkdownGroup, MarkdownMember};

#[derive(Debug)]
pub(crate) struct ScanResult {
    pub(crate) members: Vec<MarkdownMember>,
    pub(crate) groups: Vec<MarkdownGroup>,
}
