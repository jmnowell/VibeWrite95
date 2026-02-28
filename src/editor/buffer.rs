use tui_textarea::TextArea;
use ratatui::prelude::*;

pub struct EditorBuffer<'a> {
    pub textarea: TextArea<'a>,
    pub scroll_row: usize,
}

impl<'a> EditorBuffer<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        textarea.set_style(Style::default().fg(Color::White).bg(Color::Black));
        textarea.set_block(
            ratatui::widgets::Block::default()
        );
        Self { textarea, scroll_row: 0 }
    }

    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        let lines = if lines.is_empty() { vec![String::new()] } else { lines };
        let mut textarea = TextArea::new(lines);
        textarea.set_cursor_line_style(Style::default());
        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        textarea.set_style(Style::default().fg(Color::White).bg(Color::Black));
        textarea.set_block(
            ratatui::widgets::Block::default()
        );
        Self { textarea, scroll_row: 0 }
    }

    pub fn cursor_line(&self) -> usize {
        self.textarea.cursor().0
    }

    pub fn cursor_col(&self) -> usize {
        self.textarea.cursor().1
    }

    pub fn content(&self) -> String {
        self.textarea.lines().join("\n")
    }

    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.textarea.lines().len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.textarea.lines().len() <= 1 && self.textarea.lines().first().map_or(true, |l| l.is_empty())
    }
}
