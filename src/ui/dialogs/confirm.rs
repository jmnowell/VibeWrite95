use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use super::{centered_rect, dialog_block, render_clear};

pub struct ConfirmDialogState {
    pub title: String,
    pub message: String,
}

impl ConfirmDialogState {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &ConfirmDialogState) {
    let dialog_area = centered_rect(50, 6, area);
    render_clear(f, dialog_area);

    let block = dialog_block(&state.title);
    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    let msg = Paragraph::new(state.message.as_str())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(msg, Rect::new(inner.x, inner.y, inner.width, 1));

    let options = Line::from(vec![
        Span::styled("(Y)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
        Span::styled("es  ", Style::default().fg(Color::White).bg(Color::DarkGray)),
        Span::styled("(N)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
        Span::styled("o  ", Style::default().fg(Color::White).bg(Color::DarkGray)),
        Span::styled("(C)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
        Span::styled("ancel", Style::default().fg(Color::White).bg(Color::DarkGray)),
    ]);
    let options_p = Paragraph::new(options).alignment(Alignment::Center);
    if inner.height > 2 {
        f.render_widget(options_p, Rect::new(inner.x, inner.y + 2, inner.width, 1));
    }
}
