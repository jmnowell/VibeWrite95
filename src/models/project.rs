use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub version: u32,
    #[serde(rename = "fileOrder")]
    pub file_order: Vec<String>,
    #[serde(skip)]
    pub directory_path: PathBuf,
}

impl Project {
    pub fn new(directory_path: PathBuf) -> Self {
        Self {
            version: 1,
            file_order: Vec::new(),
            directory_path,
        }
    }

    pub fn name(&self) -> String {
        self.directory_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Project")
            .to_string()
    }

    pub fn project_json_path(&self) -> PathBuf {
        self.directory_path.join("project.json")
    }
}
