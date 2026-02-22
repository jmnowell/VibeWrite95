use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::models::project::Project;
use crate::models::project_file::ProjectFile;
use crate::models::editor_tab::EditorTab;

pub fn create_project(directory_path: PathBuf) -> io::Result<Project> {
    fs::create_dir_all(&directory_path)?;
    let project = Project::new(directory_path);
    save_project(&project)?;
    Ok(project)
}

pub fn load_project(directory_path: &Path) -> io::Result<Project> {
    let json_path = directory_path.join("project.json");
    let json = fs::read_to_string(&json_path)?;
    let mut project: Project = serde_json::from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    project.directory_path = directory_path.to_path_buf();
    Ok(project)
}

pub fn save_project(project: &Project) -> io::Result<()> {
    let json = serde_json::to_string_pretty(project)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(project.project_json_path(), json)
}

pub fn add_file(project: &mut Project, relative_path: &str) -> io::Result<()> {
    if !project.file_order.contains(&relative_path.to_string()) {
        project.file_order.push(relative_path.to_string());
        save_project(project)?;
    }
    Ok(())
}

pub fn remove_file(project: &mut Project, relative_path: &str) -> io::Result<()> {
    project.file_order.retain(|p| p != relative_path);
    save_project(project)
}

pub fn move_file_up(project: &mut Project, index: usize) -> io::Result<()> {
    if index > 0 && index < project.file_order.len() {
        project.file_order.swap(index, index - 1);
        save_project(project)?;
    }
    Ok(())
}

pub fn move_file_down(project: &mut Project, index: usize) -> io::Result<()> {
    if index + 1 < project.file_order.len() {
        project.file_order.swap(index, index + 1);
        save_project(project)?;
    }
    Ok(())
}

pub fn load_tabs(project: &Project) -> Vec<EditorTab> {
    project
        .file_order
        .iter()
        .map(|rel| {
            let pf = ProjectFile::new(rel, &project.directory_path);
            let content = fs::read_to_string(pf.absolute_path()).unwrap_or_default();
            EditorTab::new(pf, content)
        })
        .collect()
}

#[allow(dead_code)]
pub fn get_project_files(project: &Project) -> Vec<ProjectFile> {
    project
        .file_order
        .iter()
        .map(|rel| ProjectFile::new(rel, &project.directory_path))
        .collect()
}
