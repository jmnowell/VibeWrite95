use std::path::{Path, PathBuf};

pub struct Document {
    pub file_path: Option<PathBuf>,
}

impl Document {
    pub fn new() -> Self {
        Self { file_path: None }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            file_path: Some(path.as_ref().to_path_buf()),
        }
    }

    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}
