use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

pub fn list_files_in_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

pub fn complete_path(partial: &str) -> Vec<String> {
    let path = Path::new(partial);
    let (dir, prefix) = if partial.ends_with('/') || partial.ends_with(std::path::MAIN_SEPARATOR) {
        (PathBuf::from(partial), String::new())
    } else {
        let dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let prefix = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        (dir, prefix)
    };

    let mut completions = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(&prefix) {
                let full = dir.join(&name);
                let mut s = full.to_string_lossy().to_string();
                if full.is_dir() {
                    s.push('/');
                }
                completions.push(s);
            }
        }
    }
    completions.sort();
    completions
}
