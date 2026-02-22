use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectFile {
    pub relative_path: String,
    pub project_dir: PathBuf,
}

impl ProjectFile {
    pub fn new(relative_path: &str, project_dir: &Path) -> Self {
        Self {
            relative_path: relative_path.to_string(),
            project_dir: project_dir.to_path_buf(),
        }
    }

    pub fn absolute_path(&self) -> PathBuf {
        self.project_dir.join(&self.relative_path)
    }

    pub fn file_name(&self) -> &str {
        Path::new(&self.relative_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.relative_path)
    }
}
