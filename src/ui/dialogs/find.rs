use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use super::{centered_rect, dialog_block, render_clear};

#[derive(Debug, Clone, PartialEq)]
pub enum FindMode {
    Find,
    Replace,
}

pub struct FindDialogState {
    pub mode: FindMode,
    pub search_term: String,
    pub replace_term: String,
    pub search_cursor: usize,
    pub replace_cursor: usize,
    pub active_field: FindField,
    pub case_sensitive: bool,
    pub last_match_offset: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FindField {
    Search,
    Replace,
}

impl FindDialogState {
    pub fn new_find() -> Self {
        Self {
            mode: FindMode::Find,
            search_term: String::new(),
            replace_term: String::new(),
            search_cursor: 0,
            replace_cursor: 0,
            active_field: FindField::Search,
            case_sensitive: false,
            last_match_offset: None,
        }
    }

    pub fn new_replace() -> Self {
        let mut s = Self::new_find();
        s.mode = FindMode::Replace;
        s
    }

    pub fn insert_char(&mut self, c: char) {
        match self.active_field {
            FindField::Search => {
                self.search_term.insert(self.search_cursor, c);
                self.search_cursor += c.len_utf8();
            }
            FindField::Replace => {
                self.replace_term.insert(self.replace_cursor, c);
                self.replace_cursor += c.len_utf8();
            }
        }
    }

    pub fn backspace(&mut self) {
        match self.active_field {
            FindField::Search => {
                if self.search_cursor > 0 {
                    let prev = self.search_term[..self.search_cursor].chars().last().map_or(0, |c| c.len_utf8());
                    self.search_cursor -= prev;
                    self.search_term.remove(self.search_cursor);
                }
            }
            FindField::Replace => {
                if self.replace_cursor > 0 {
                    let prev = self.replace_term[..self.replace_cursor].chars().last().map_or(0, |c| c.len_utf8());
                    self.replace_cursor -= prev;
                    self.replace_term.remove(self.replace_cursor);
                }
            }
        }
    }

    pub fn find_next(&mut self, text: &str) -> Option<usize> {
        let start = self.last_match_offset.map(|o| o + 1).unwrap_or(0);
        let search = if self.case_sensitive {
            self.search_term.clone()
        } else {
            self.search_term.to_lowercase()
        };
        let haystack = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        if let Some(pos) = haystack[start..].find(&search) {
            let abs_pos = start + pos;
            self.last_match_offset = Some(abs_pos);
            Some(abs_pos)
        } else if start > 0 {
            // Wrap around
            if let Some(pos) = haystack.find(&search) {
                self.last_match_offset = Some(pos);
                Some(pos)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn replace_current(&self, text: &str) -> Option<String> {
        self.last_match_offset.map(|offset| {
            let mut result = text.to_string();
            result.replace_range(offset..offset + self.search_term.len(), &self.replace_term);
            result
        })
    }

    pub fn replace_all(&self, text: &str) -> String {
        if self.case_sensitive {
            text.replace(&self.search_term, &self.replace_term)
        } else {
            // Case-insensitive replace
            let mut result = String::with_capacity(text.len());
            let lower_search = self.search_term.to_lowercase();
            let lower_text = text.to_lowercase();
            let mut last = 0;
            let mut pos = 0;
            while pos < lower_text.len() {
                if lower_text[pos..].starts_with(&lower_search) {
                    result.push_str(&text[last..pos]);
                    result.push_str(&self.replace_term);
                    pos += self.search_term.len();
                    last = pos;
                } else {
                    pos += 1;
                }
            }
            result.push_str(&text[last..]);
            result
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &FindDialogState) {
    let height = if state.mode == FindMode::Replace { 7 } else { 5 };
    let dialog_area = centered_rect(60, height, area);
    render_clear(f, dialog_area);

    let title = if state.mode == FindMode::Find { "Find" } else { "Find & Replace" };
    let block = dialog_block(title);
    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    // Search field
    let search_label = Paragraph::new("Find:   ")
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(search_label, Rect::new(inner.x, inner.y, 8, 1));

    let search_input = Paragraph::new(state.search_term.as_str())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    f.render_widget(search_input, Rect::new(inner.x + 8, inner.y, inner.width - 8, 1));

    if state.active_field == FindField::Search {
        f.set_cursor_position((inner.x + 8 + state.search_cursor as u16, inner.y));
    }

    if state.mode == FindMode::Replace && inner.height > 1 {
        let replace_label = Paragraph::new("Replace:")
            .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
        f.render_widget(replace_label, Rect::new(inner.x, inner.y + 1, 8, 1));

        let replace_input = Paragraph::new(state.replace_term.as_str())
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));
        f.render_widget(replace_input, Rect::new(inner.x + 8, inner.y + 1, inner.width - 8, 1));

        if state.active_field == FindField::Replace {
            f.set_cursor_position((inner.x + 8 + state.replace_cursor as u16, inner.y + 1));
        }

        if inner.height > 3 {
            let hint = Paragraph::new("Enter: Find Next  Tab: field  A: All  Esc: Close")
                .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
            f.render_widget(hint, Rect::new(inner.x, inner.y + 3, inner.width, 1));
        }
    } else if inner.height > 2 {
        let hint = Paragraph::new("Enter: Find Next  Esc: Close")
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
        f.render_widget(hint, Rect::new(inner.x, inner.y + 2, inner.width, 1));
    }
}
