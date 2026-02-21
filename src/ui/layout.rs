use ratatui::prelude::*;
use crate::app::App;

pub struct LayoutAreas {
    pub help: Rect,
    pub status: Rect,
    pub ruler: Rect,
    pub outline: Option<Rect>,
    pub editor: Rect,
    pub project: Option<Rect>,
    pub tab_bar: Option<Rect>,
}

pub fn compute_layout(area: Rect, app: &App) -> LayoutAreas {
    let help_height = if app.show_help { 4 } else { 0 };
    let ruler_height: u16 = if app.show_ruler { 1 } else { 0 };
    let status_height: u16 = 1;
    let tab_bar_height: u16 = if app.project_name.is_some() { 1 } else { 0 };

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(help_height),
            Constraint::Length(status_height),
            Constraint::Length(ruler_height),
            Constraint::Min(1),
            Constraint::Length(tab_bar_height),
        ])
        .split(area);

    let main_area = vertical[3];

    // Horizontal split for outline/editor/project
    let outline_width: u16 = if app.show_outline { 22 } else { 0 };
    let project_width: u16 = if app.show_project { 22 } else { 0 };

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(outline_width),
            Constraint::Min(1),
            Constraint::Length(project_width),
        ])
        .split(main_area);

    LayoutAreas {
        help: vertical[0],
        status: vertical[1],
        ruler: vertical[2],
        outline: if app.show_outline { Some(horizontal[0]) } else { None },
        editor: horizontal[1],
        project: if app.show_project { Some(horizontal[2]) } else { None },
        tab_bar: if app.project_name.is_some() { Some(vertical[4]) } else { None },
    }
}
