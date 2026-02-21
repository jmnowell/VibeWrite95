use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::app::{App, AppMode, PrefixKey};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    if area.height == 0 {
        return;
    }

    let lines = match app.mode {
        AppMode::CommandPrefix(PrefixKey::CtrlK) => ctrl_k_help(),
        AppMode::CommandPrefix(PrefixKey::CtrlQ) => ctrl_q_help(),
        AppMode::CommandPrefix(PrefixKey::CtrlO) => ctrl_o_help(),
        _ => main_help(),
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(paragraph, area);
}

fn key_span(key: &str) -> Span<'static> {
    Span::styled(
        key.to_string(),
        Style::default().fg(Color::Yellow).bg(Color::Blue).add_modifier(Modifier::BOLD),
    )
}

fn text_span(text: &str) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(Color::White).bg(Color::Blue),
    )
}

fn sep() -> Span<'static> {
    Span::styled(
        " | ".to_string(),
        Style::default().fg(Color::Cyan).bg(Color::Blue),
    )
}

fn main_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            text_span(" "), key_span("^N"), text_span(" New  "),
            key_span("^O"), text_span(" Open  "),
            key_span("^S"), text_span(" Save  "),
            key_span("^A"), text_span(" SaveAs"),
            sep(),
            key_span("^B"), text_span(" Bold  "),
            key_span("^I"), text_span(" Italic  "),
            key_span("^U"), text_span(" Underline"),
        ]),
        Line::from(vec![
            text_span(" "), key_span("^K"), text_span(" Block/File  "),
            key_span("^Q"), text_span(" Quick/Search"),
            sep(),
            key_span("^P"), text_span(" Print  "),
            key_span("F7"), text_span(" Spell  "),
            key_span("^W"), text_span(" Close"),
        ]),
        Line::from(vec![
            text_span(" "), key_span("^K"), text_span(": S Save  Q Quit  P Print  N NewProj  O OpenProj"),
        ]),
        Line::from(vec![
            text_span(" "), key_span("^Q"), text_span(": F Find  A Replace  C Commit  H History  V Revert"),
        ]),
    ]
}

fn ctrl_k_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            text_span(" "),
            key_span("^K"),
            text_span("  Block & File Commands:"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("B"), text_span(" Begin block  "),
            key_span("K"), text_span(" End block  "),
            key_span("V"), text_span(" Move  "),
            key_span("C"), text_span(" Copy  "),
            key_span("Y"), text_span(" Delete block"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("S"), text_span(" Save  "),
            key_span("D"), text_span(" Save&Done  "),
            key_span("Q"), text_span(" Quit  "),
            key_span("H"), text_span(" Hide block"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("N"), text_span(" New project  "),
            key_span("O"), text_span(" Open project  "),
            key_span("P"), text_span(" Print  "),
            key_span("X"), text_span(" Export PDF  "),
            key_span("1-9"), text_span(" Tab"),
        ]),
    ]
}

fn ctrl_q_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            text_span(" "),
            key_span("^Q"),
            text_span("  Quick Commands:"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("F"), text_span(" Find text  "),
            key_span("A"), text_span(" Find & Replace  "),
            key_span("R"), text_span(" Repeat last find"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("C"), text_span(" Commit  "),
            key_span("H"), text_span(" View history  "),
            key_span("V"), text_span(" Revert to version"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("S"), text_span(" Start of file  "),
            key_span("D"), text_span(" End of file  "),
            key_span("L"), text_span(" Go to line"),
        ]),
    ]
}

fn ctrl_o_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            text_span(" "),
            key_span("^O"),
            text_span("  Onscreen/View Commands:"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("L"), text_span(" Toggle outline  "),
            key_span("P"), text_span(" Toggle project pane  "),
            key_span("W"), text_span(" Toggle word wrap"),
        ]),
        Line::from(vec![
            text_span(" "),
            key_span("N"), text_span(" Toggle line numbers  "),
            key_span("R"), text_span(" Toggle ruler  "),
            key_span("T"), text_span(" Next tab"),
        ]),
        Line::from(vec![
            text_span(" "),
        ]),
    ]
}
