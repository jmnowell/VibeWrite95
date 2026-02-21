use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use super::{centered_rect, dialog_block, render_clear};

pub struct FileDialogState {
    pub input: String,
    pub cursor_pos: usize,
    pub title: String,
    pub completions: Vec<String>,
    pub selected_completion: usize,
}

impl FileDialogState {
    pub fn new(title: &str) -> Self {
        Self {
            input: String::new(),
            cursor_pos: 0,
            title: title.to_string(),
            completions: Vec::new(),
            selected_completion: 0,
        }
    }

    pub fn with_initial(title: &str, initial: &str) -> Self {
        Self {
            input: initial.to_string(),
            cursor_pos: initial.len(),
            title: title.to_string(),
            completions: Vec::new(),
            selected_completion: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.input[..self.cursor_pos]
                .chars()
                .last()
                .map_or(0, |c| c.len_utf8());
            self.cursor_pos -= prev;
            self.input.remove(self.cursor_pos);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.input[..self.cursor_pos]
                .chars()
                .last()
                .map_or(0, |c| c.len_utf8());
            self.cursor_pos -= prev;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            let next = self.input[self.cursor_pos..]
                .chars()
                .next()
                .map_or(0, |c| c.len_utf8());
            self.cursor_pos += next;
        }
    }

    pub fn tab_complete(&mut self) {
        let completions = crate::services::file_service::complete_path(&self.input);
        if completions.len() == 1 {
            self.input = completions[0].clone();
            self.cursor_pos = self.input.len();
        }
        self.completions = completions;
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &FileDialogState) {
    let dialog_area = centered_rect(60, 5, area);
    render_clear(f, dialog_area);

    let block = dialog_block(&state.title);
    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    let input_line = format!("> {}", state.input);
    let prompt = Paragraph::new(input_line)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    f.render_widget(prompt, inner);

    // Show cursor position
    let cursor_x = inner.x + 2 + state.cursor_pos as u16;
    let cursor_y = inner.y;
    f.set_cursor_position((cursor_x, cursor_y));

    // Show hint
    if inner.height > 1 {
        let hint = Paragraph::new("Enter: confirm  Tab: complete  Esc: cancel")
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
        let hint_area = Rect::new(inner.x, inner.y + 1, inner.width, 1);
        f.render_widget(hint, hint_area);
    }
}
