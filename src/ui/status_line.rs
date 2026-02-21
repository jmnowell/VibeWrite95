use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    if area.height == 0 {
        return;
    }

    let mode_str = if app.insert_mode { "Insert" } else { "Overwrite" };
    let wrap_str = if app.word_wrap { "Wrap" } else { "" };
    let file_name = app.document.file_name();
    let dirty = if app.is_dirty { "*" } else { "" };

    let left = format!(
        " L:{} C:{}  {}  {}  {} {}",
        app.cursor_line + 1,
        app.cursor_col + 1,
        mode_str,
        wrap_str,
        file_name,
        dirty,
    );

    let right = "VibeWrite95 ";
    let project_info = if let Some(ref name) = app.project_name {
        format!("[Project: {}]  ", name)
    } else {
        String::new()
    };

    let padding = if area.width as usize > left.len() + right.len() + project_info.len() {
        area.width as usize - left.len() - right.len() - project_info.len()
    } else {
        1
    };

    let line = format!("{}{:>pad$}{}{}", left, "", project_info, right, pad = padding);

    let paragraph = Paragraph::new(line)
        .style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_widget(paragraph, area);
}
