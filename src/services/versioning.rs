use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::Utc;
use rand::Rng;

use crate::models::version::{VersionHistory, VersionCommit};

pub fn vibe_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(".vibe")
}

pub fn file_vibe_dir(project_dir: &Path, filename: &str) -> PathBuf {
    vibe_dir(project_dir).join(filename)
}

pub fn snapshots_dir(project_dir: &Path, filename: &str) -> PathBuf {
    file_vibe_dir(project_dir, filename).join("snapshots")
}

pub fn history_path(project_dir: &Path, filename: &str) -> PathBuf {
    file_vibe_dir(project_dir, filename).join("history.json")
}

pub fn generate_commit_id() -> String {
    let now = Utc::now();
    let mut rng = rand::rng();
    let hex: u16 = rng.random();
    format!("{}-{:04x}", now.format("%Y%m%d-%H%M%S"), hex)
}

pub fn load_history(project_dir: &Path, filename: &str) -> VersionHistory {
    let path = history_path(project_dir, filename);
    if let Ok(json) = fs::read_to_string(&path) {
        serde_json::from_str(&json).unwrap_or_else(|_| VersionHistory::new())
    } else {
        VersionHistory::new()
    }
}

pub fn save_history(project_dir: &Path, filename: &str, history: &VersionHistory) -> io::Result<()> {
    let path = history_path(project_dir, filename);
    fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(history)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, json)
}

pub fn commit(
    project_dir: &Path,
    filename: &str,
    content: &str,
    message: &str,
) -> io::Result<String> {
    let commit_id = generate_commit_id();
    let snap_dir = snapshots_dir(project_dir, filename);
    fs::create_dir_all(&snap_dir)?;

    let snap_path = snap_dir.join(format!("{}.md", commit_id));
    fs::write(&snap_path, content)?;

    let mut history = load_history(project_dir, filename);
    let commit = VersionCommit {
        id: commit_id.clone(),
        timestamp: Utc::now().to_rfc3339(),
        message: message.to_string(),
        file_size_bytes: content.len() as u64,
    };
    history.commits.push(commit);
    save_history(project_dir, filename, &history)?;

    Ok(commit_id)
}

pub fn load_snapshot(project_dir: &Path, filename: &str, commit_id: &str) -> io::Result<String> {
    let path = snapshots_dir(project_dir, filename).join(format!("{}.md", commit_id));
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_commit_and_load() {
        let dir = env::temp_dir().join("vibewrite_test");
        fs::create_dir_all(&dir).unwrap();
        let id = commit(&dir, "test.md", "Hello, world!", "Test commit").unwrap();
        let content = load_snapshot(&dir, "test.md", &id).unwrap();
        assert_eq!(content, "Hello, world!");
        // Cleanup
        let _ = fs::remove_dir_all(&dir);
    }
}
