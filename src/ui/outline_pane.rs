use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use crate::models::outline_item::OutlineItem;

pub struct OutlinePaneState {
    pub items: Vec<OutlineItem>,
    pub list_state: ListState,
}

impl OutlinePaneState {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            list_state: ListState::default(),
        }
    }

    pub fn update(&mut self, items: Vec<OutlineItem>) {
        self.items = items;
        if self.items.is_empty() {
            self.list_state.select(None);
        } else if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 { self.items.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn selected_item(&self) -> Option<&OutlineItem> {
        self.list_state.selected().and_then(|i| self.items.get(i))
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut OutlinePaneState, focused: bool) {
    let border_style = if focused {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Cyan).bg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Outline ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));

    let items: Vec<ListItem> = state
        .items
        .iter()
        .map(|item| {
            let indent = "  ".repeat((item.level - 1) as usize);
            let prefix = match item.level {
                1 => "# ",
                2 => "## ",
                3 => "### ",
                _ => "",
            };
            let style = match item.level {
                1 => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                2 => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                _ => Style::default().fg(Color::Cyan),
            };
            ListItem::new(format!("{}{}{}", indent, prefix, item.title)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut state.list_state);
}
