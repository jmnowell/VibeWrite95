use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use super::{centered_rect, dialog_block, render_clear};

pub struct InputDialogState {
    pub title: String,
    pub prompt: String,
    pub input: String,
    pub cursor_pos: usize,
}

impl InputDialogState {
    pub fn new(title: &str, prompt: &str) -> Self {
        Self {
            title: title.to_string(),
            prompt: prompt.to_string(),
            input: String::new(),
            cursor_pos: 0,
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
}

pub fn render(f: &mut Frame, area: Rect, state: &InputDialogState) {
    let dialog_area = centered_rect(50, 5, area);
    render_clear(f, dialog_area);

    let block = dialog_block(&state.title);
    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    let prompt_line = Paragraph::new(state.prompt.as_str())
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(prompt_line, Rect::new(inner.x, inner.y, inner.width, 1));

    if inner.height > 1 {
        let input_line = format!("> {}", state.input);
        let input_p = Paragraph::new(input_line)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));
        f.render_widget(input_p, Rect::new(inner.x, inner.y + 1, inner.width, 1));

        let cursor_x = inner.x + 2 + state.cursor_pos as u16;
        let cursor_y = inner.y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
