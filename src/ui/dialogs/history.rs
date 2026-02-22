use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use crate::models::version::VersionCommit;
use super::{centered_rect, render_clear};

pub struct HistoryDialogState {
    pub commits: Vec<VersionCommit>,
    pub list_state: ListState,
    pub filename: String,
}

impl HistoryDialogState {
    pub fn new(filename: &str, commits: Vec<VersionCommit>) -> Self {
        let mut list_state = ListState::default();
        if !commits.is_empty() {
            list_state.select(Some(commits.len() - 1));
        }
        Self { commits, list_state, filename: filename.to_string() }
    }

    pub fn next(&mut self) {
        if self.commits.is_empty() { return; }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.commits.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.commits.is_empty() { return; }
        let i = match self.list_state.selected() {
            Some(i) => if i == 0 { self.commits.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn selected_commit(&self) -> Option<&VersionCommit> {
        self.list_state.selected().and_then(|i| self.commits.get(i))
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut HistoryDialogState) {
    let dialog_area = centered_rect(70, 20, area);
    render_clear(f, dialog_area);

    let title = format!(" Version History: {} ", state.filename);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    let header = Paragraph::new("  ID                    Date                  Message            Size")
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(header, Rect::new(inner.x, inner.y, inner.width, 1));

    let sep = Paragraph::new("  ─────────────────────────────────────────────────────────────────")
        .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
    f.render_widget(sep, Rect::new(inner.x, inner.y + 1, inner.width, 1));

    let list_area = Rect::new(inner.x, inner.y + 2, inner.width, inner.height.saturating_sub(4));

    let items: Vec<ListItem> = state.commits.iter().map(|c| {
        let date = c.timestamp.get(0..16).unwrap_or(&c.timestamp);
        let size_kb = c.file_size_bytes / 1024;
        let text = format!(
            "  {}  {}  {:30}  {} KB",
            &c.id, date, &c.message, size_kb
        );
        ListItem::new(text).style(Style::default().fg(Color::White).bg(Color::DarkGray))
    }).collect();

    let list = List::new(items)
        .highlight_style(
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, list_area, &mut state.list_state);

    let hint = Paragraph::new("  Enter: Select/Revert    Esc: Close")
        .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
    let hint_y = inner.y + inner.height.saturating_sub(1);
    f.render_widget(hint, Rect::new(inner.x, hint_y, inner.width, 1));
}
