use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use crate::models::spell::MisspelledWord;
use super::{centered_rect, render_clear};

pub struct SpellCheckDialogState {
    pub words: Vec<MisspelledWord>,
    pub word_list_state: ListState,
    pub suggestion_list_state: ListState,
    pub focus: SpellFocus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpellFocus {
    Words,
    Suggestions,
}

impl SpellCheckDialogState {
    pub fn new(words: Vec<MisspelledWord>) -> Self {
        let mut word_list_state = ListState::default();
        if !words.is_empty() {
            word_list_state.select(Some(0));
        }
        Self {
            words,
            word_list_state,
            suggestion_list_state: ListState::default(),
            focus: SpellFocus::Words,
        }
    }

    pub fn selected_word(&self) -> Option<&MisspelledWord> {
        self.word_list_state.selected().and_then(|i| self.words.get(i))
    }

    pub fn selected_suggestion(&self) -> Option<String> {
        let word = self.selected_word()?;
        let idx = self.suggestion_list_state.selected()?;
        word.suggestions.get(idx).cloned()
    }

    pub fn next_word(&mut self) {
        if self.words.is_empty() { return; }
        let i = match self.word_list_state.selected() {
            Some(i) => (i + 1) % self.words.len(),
            None => 0,
        };
        self.word_list_state.select(Some(i));
        self.suggestion_list_state.select(None);
    }

    pub fn previous_word(&mut self) {
        if self.words.is_empty() { return; }
        let i = match self.word_list_state.selected() {
            Some(i) => if i == 0 { self.words.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.word_list_state.select(Some(i));
        self.suggestion_list_state.select(None);
    }

    #[allow(dead_code)]
    pub fn next_suggestion(&mut self) {
        let count = self.selected_word().map_or(0, |w| w.suggestions.len());
        if count == 0 { return; }
        let i = match self.suggestion_list_state.selected() {
            Some(i) => (i + 1) % count,
            None => 0,
        };
        self.suggestion_list_state.select(Some(i));
    }

    #[allow(dead_code)]
    pub fn previous_suggestion(&mut self) {
        let count = self.selected_word().map_or(0, |w| w.suggestions.len());
        if count == 0 { return; }
        let i = match self.suggestion_list_state.selected() {
            Some(i) => if i == 0 { count - 1 } else { i - 1 },
            None => 0,
        };
        self.suggestion_list_state.select(Some(i));
    }

    pub fn remove_selected_word(&mut self) {
        if let Some(i) = self.word_list_state.selected() {
            if i < self.words.len() {
                self.words.remove(i);
                if i >= self.words.len() && i > 0 {
                    self.word_list_state.select(Some(i - 1));
                } else if self.words.is_empty() {
                    self.word_list_state.select(None);
                }
            }
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut SpellCheckDialogState) {
    let dialog_area = centered_rect(72, 20, area);
    render_clear(f, dialog_area);

    let block = Block::default()
        .title(" Spell Check ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    let inner = block.inner(dialog_area);
    f.render_widget(block, dialog_area);

    let left_w = inner.width / 2;
    let right_w = inner.width - left_w;
    let left_area = Rect::new(inner.x, inner.y, left_w, inner.height.saturating_sub(2));
    let right_area = Rect::new(inner.x + left_w, inner.y, right_w, inner.height.saturating_sub(2));

    // Left pane: misspelled words
    let word_title = if state.focus == SpellFocus::Words { " [Words] " } else { " Words " };
    let word_block = Block::default()
        .title(word_title)
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    let word_inner = word_block.inner(left_area);
    f.render_widget(word_block, left_area);

    let word_items: Vec<ListItem> = state.words.iter().map(|w| {
        ListItem::new(format!("  {} ({})", w.word, w.occurrence_count()))
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
    }).collect();

    let word_list = List::new(word_items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(word_list, word_inner, &mut state.word_list_state);

    // Right pane: details & suggestions
    if let Some(word) = state.selected_word() {
        let word_detail = format!("  Word: \"{}\"  ({} occurrences)", word.word, word.occurrence_count());
        let detail_p = Paragraph::new(word_detail)
            .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray));
        f.render_widget(detail_p, Rect::new(right_area.x, right_area.y, right_area.width, 1));

        let sugg_label = Paragraph::new("  Suggestions:")
            .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
        f.render_widget(sugg_label, Rect::new(right_area.x, right_area.y + 1, right_area.width, 1));

        let sugg_items: Vec<ListItem> = word.suggestions.iter().map(|s| {
            ListItem::new(format!("    {}", s))
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        }).collect();

        let sugg_list = List::new(sugg_items)
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD));
        let sugg_area = Rect::new(right_area.x, right_area.y + 2, right_area.width,
            right_area.height.saturating_sub(5));
        f.render_stateful_widget(sugg_list, sugg_area, &mut state.suggestion_list_state);

        // Action keys
        let actions = Line::from(vec![
            Span::styled("(R)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled("eplace ", Style::default().fg(Color::White).bg(Color::DarkGray)),
            Span::styled("(I)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled("gnore ", Style::default().fg(Color::White).bg(Color::DarkGray)),
            Span::styled("(D)", Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled("ict ", Style::default().fg(Color::White).bg(Color::DarkGray)),
        ]);
        let action_y = right_area.y + right_area.height.saturating_sub(3);
        f.render_widget(Paragraph::new(actions), Rect::new(right_area.x, action_y, right_area.width, 1));
    }

    // Bottom hint
    let hint_y = inner.y + inner.height.saturating_sub(1);
    let hint = Paragraph::new("  Tab: switch pane   Esc: Close")
        .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
    f.render_widget(hint, Rect::new(inner.x, hint_y, inner.width, 1));
}
