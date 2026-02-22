#[derive(Debug, Clone)]
pub struct OutlineItem {
    pub title: String,
    pub level: u8,
    pub line_number: usize,
    #[allow(dead_code)]
    pub source_file_path: Option<String>,
}
