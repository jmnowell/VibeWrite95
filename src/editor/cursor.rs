#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CursorPosition {
    pub line: usize,
    pub col: usize,
}

impl CursorPosition {
    #[allow(dead_code)]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
