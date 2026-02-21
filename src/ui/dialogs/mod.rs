pub mod file_dialog;
pub mod confirm;
pub mod input;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

pub fn dialog_block(title: &str) -> Block<'_> {
    Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
}

pub fn render_clear(f: &mut Frame, area: Rect) {
    f.render_widget(Clear, area);
}
