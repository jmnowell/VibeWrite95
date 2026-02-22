use crate::models::project_file::ProjectFile;

#[derive(Debug, Clone)]
pub struct EditorTab {
    pub project_file: ProjectFile,
    pub content: String,
    #[allow(dead_code)]
    pub caret_offset: usize,
    #[allow(dead_code)]
    pub scroll_offset: usize,
    pub is_dirty: bool,
}

impl EditorTab {
    pub fn new(project_file: ProjectFile, content: String) -> Self {
        Self {
            project_file,
            content,
            caret_offset: 0,
            scroll_offset: 0,
            is_dirty: false,
        }
    }

    pub fn display_name(&self) -> String {
        if self.is_dirty {
            format!("{}*", self.project_file.file_name())
        } else {
            self.project_file.file_name().to_string()
        }
    }
}
