use tui_textarea::TextArea;

pub fn toggle_bold(textarea: &mut TextArea) {
    toggle_wrap(textarea, "**");
}

pub fn toggle_italic(textarea: &mut TextArea) {
    toggle_wrap(textarea, "*");
}

pub fn toggle_underline(textarea: &mut TextArea) {
    toggle_wrap(textarea, "++");
}

pub fn cycle_heading(textarea: &mut TextArea) {
    let (row, _col) = textarea.cursor();
    let line = textarea.lines()[row].clone();
    let trimmed = line.trim_start();

    let new_line = if trimmed.starts_with("### ") {
        // H3 -> Normal
        trimmed.strip_prefix("### ").unwrap_or(trimmed).to_string()
    } else if trimmed.starts_with("## ") {
        // H2 -> H3
        format!("### {}", trimmed.strip_prefix("## ").unwrap_or(trimmed))
    } else if trimmed.starts_with("# ") {
        // H1 -> H2
        format!("## {}", trimmed.strip_prefix("# ").unwrap_or(trimmed))
    } else {
        // Normal -> H1
        format!("# {}", trimmed)
    };

    // Select entire line and replace
    textarea.move_cursor(tui_textarea::CursorMove::Head);
    textarea.move_cursor(tui_textarea::CursorMove::End);
    // Select from beginning to end
    textarea.move_cursor(tui_textarea::CursorMove::Head);
    textarea.start_selection();
    textarea.move_cursor(tui_textarea::CursorMove::End);
    textarea.insert_str(&new_line);
}

fn toggle_wrap(textarea: &mut TextArea, delimiter: &str) {
    // Get selection or current word
    if let Some((start, end)) = textarea.selection_range() {
        let selected_text: String = textarea.lines()[start.0..=end.0]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let s = if i == 0 { start.1 } else { 0 };
                let e = if i == end.0 - start.0 { end.1 } else { line.len() };
                &line[s..e]
            })
            .collect::<Vec<&str>>()
            .join("\n");

        // Check if already wrapped
        if selected_text.starts_with(delimiter) && selected_text.ends_with(delimiter) {
            let unwrapped = &selected_text[delimiter.len()..selected_text.len() - delimiter.len()];
            textarea.insert_str(unwrapped);
        } else {
            let wrapped = format!("{}{}{}", delimiter, selected_text, delimiter);
            textarea.insert_str(&wrapped);
        }
    } else {
        // No selection, wrap/unwrap at cursor
        textarea.insert_str(delimiter);
        textarea.insert_str(delimiter);
        // Move cursor back between delimiters
        for _ in 0..delimiter.len() {
            textarea.move_cursor(tui_textarea::CursorMove::Back);
        }
    }
}
