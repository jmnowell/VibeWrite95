use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistory {
    pub version: u32,
    pub commits: Vec<VersionCommit>,
}

impl VersionHistory {
    pub fn new() -> Self {
        Self { version: 1, commits: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCommit {
    pub id: String,
    pub timestamp: String,
    pub message: String,
    #[serde(rename = "fileSizeBytes")]
    pub file_size_bytes: u64,
}
