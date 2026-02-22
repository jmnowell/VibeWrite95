use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::models::editor_tab::EditorTab;

pub fn render(f: &mut Frame, area: Rect, tabs: &[EditorTab], active_tab: usize) {
    if area.height == 0 || tabs.is_empty() {
        return;
    }

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(" ", Style::default().fg(Color::White).bg(Color::DarkGray)));

    for (i, tab) in tabs.iter().enumerate() {
        let name = tab.display_name();
        if i == active_tab {
            spans.push(Span::styled(
                format!(" {} ", name),
                Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                format!(" {} ", name),
                Style::default().fg(Color::White).bg(Color::DarkGray),
            ));
        }
        if i + 1 < tabs.len() {
            spans.push(Span::styled(
                "|",
                Style::default().fg(Color::DarkGray).bg(Color::DarkGray),
            ));
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    f.render_widget(paragraph, area);
}
