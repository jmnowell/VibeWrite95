use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::services::line_diff::{diff_lines, DiffLine};
use super::{centered_rect, render_clear};

pub struct DiffViewState {
    pub commit_id: String,
    pub commit_message: String,
    pub filename: String,
    pub current_content: String,
    pub snapshot_content: String,
    pub scroll: u16,
}

impl DiffViewState {
    pub fn new(
        commit_id: String,
        commit_message: String,
        filename: String,
        current_content: String,
        snapshot_content: String,
    ) -> Self {
        Self {
            commit_id,
            commit_message,
            filename,
            current_content,
            snapshot_content,
            scroll: 0,
        }
    }

    pub fn scroll_down(&mut self) { self.scroll = self.scroll.saturating_add(3); }
    pub fn scroll_up(&mut self) { self.scroll = self.scroll.saturating_sub(3); }
}

pub fn render(f: &mut Frame, area: Rect, state: &DiffViewState) {
    let dialog_area = centered_rect(90, 24, area);
    render_clear(f, dialog_area);

    let title = format!(" Revert: {} ", state.filename);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    // Commit info header
    let header = format!("  Commit: {}  \"{}\"", state.commit_id, state.commit_message);
    let header_p = Paragraph::new(header)
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(header_p, Rect::new(inner.x, inner.y, inner.width, 1));

    // Column headers
    let half_w = inner.width / 2;
    let col_header = format!(
        "  {:width$}│  Snapshot",
        "Current",
        width = (half_w as usize).saturating_sub(4)
    );
    let col_p = Paragraph::new(col_header)
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(col_p, Rect::new(inner.x, inner.y + 1, inner.width, 1));

    // Diff content
    let diff = diff_lines(&state.current_content, &state.snapshot_content);
    let diff_area = Rect::new(inner.x, inner.y + 2, inner.width, inner.height.saturating_sub(4));

    let mut lines: Vec<Line> = Vec::new();
    for (cur, snap) in &diff {
        let (cur_text, cur_style) = format_diff_line(cur, half_w as usize);
        let (snap_text, snap_style) = format_diff_line(snap, half_w as usize);
        lines.push(Line::from(vec![
            Span::styled(cur_text, cur_style.bg(Color::DarkGray)),
            Span::styled("│", Style::default().fg(Color::Cyan).bg(Color::DarkGray)),
            Span::styled(snap_text, snap_style.bg(Color::DarkGray)),
        ]));
    }

    let diff_p = Paragraph::new(lines)
        .scroll((state.scroll, 0));
    f.render_widget(diff_p, diff_area);

    // Hint
    let hint = Paragraph::new("  (R)evert to this version    PgUp/PgDn: Scroll    Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
    let hint_y = inner.y + inner.height.saturating_sub(1);
    f.render_widget(hint, Rect::new(inner.x, hint_y, inner.width, 1));
}

fn format_diff_line(line: &DiffLine, width: usize) -> (String, Style) {
    match line {
        DiffLine::Same(text) => {
            let truncated = truncate(text, width);
            (format!("  {:<width$}", truncated, width = width.saturating_sub(2)),
             Style::default().fg(Color::White))
        }
        DiffLine::Removed(text) => {
            let truncated = truncate(text, width);
            (format!("- {:<width$}", truncated, width = width.saturating_sub(2)),
             Style::default().fg(Color::Red))
        }
        DiffLine::Added(text) => {
            let truncated = truncate(text, width);
            (format!("+ {:<width$}", truncated, width = width.saturating_sub(2)),
             Style::default().fg(Color::Green))
        }
    }
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max { s } else { &s[..max] }
}
