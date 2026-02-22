use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use crate::models::editor_tab::EditorTab;

pub struct ProjectPaneState {
    pub list_state: ListState,
}

impl ProjectPaneState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn select(&mut self, index: usize) {
        self.list_state.select(Some(index));
    }

    pub fn next(&mut self, count: usize) {
        if count == 0 { return; }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % count,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self, count: usize) {
        if count == 0 { return; }
        let i = match self.list_state.selected() {
            Some(i) => if i == 0 { count - 1 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

pub fn render(
    f: &mut Frame,
    area: Rect,
    tabs: &[EditorTab],
    active_tab: usize,
    state: &mut ProjectPaneState,
    focused: bool,
) {
    let border_style = if focused {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Cyan).bg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Project ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));

    let items: Vec<ListItem> = tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            let style = if i == active_tab {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default().fg(Color::Cyan).bg(Color::DarkGray)
            };
            ListItem::new(tab.display_name()).style(style)
        })
        .collect();

    let inner = block.inner(area);
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    if state.list_state.selected().is_none() && !tabs.is_empty() {
        state.list_state.select(Some(active_tab));
    }

    f.render_stateful_widget(list, area, &mut state.list_state);

    // Render action hints at bottom
    if inner.height > 3 && !tabs.is_empty() {
        let hints_y = inner.y + inner.height - 2;
        let hints = Paragraph::new("[A]dd [R]move [U]p [D]n")
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
        use ratatui::widgets::Paragraph;
        f.render_widget(hints, Rect::new(inner.x, hints_y, inner.width, 1));
    }
}
