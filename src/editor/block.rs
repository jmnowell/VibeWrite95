use tui_textarea::TextArea;

pub fn get_block_text(textarea: &TextArea, begin: (usize, usize), end: (usize, usize)) -> String {
    let (start, finish) = normalize_range(begin, end);
    let lines = textarea.lines();
    let mut result = String::new();

    for (i, line) in lines.iter().enumerate() {
        if i < start.0 || i > finish.0 {
            continue;
        }
        let s = if i == start.0 { start.1 } else { 0 };
        let e = if i == finish.0 { finish.1.min(line.len()) } else { line.len() };

        if s <= e && s <= line.len() {
            result.push_str(&line[s..e]);
        }
        if i < finish.0 {
            result.push('\n');
        }
    }

    result
}

pub fn delete_block(textarea: &mut TextArea, begin: (usize, usize), end: (usize, usize)) {
    let (start, finish) = normalize_range(begin, end);

    // Move to start of block
    textarea.move_cursor(tui_textarea::CursorMove::Top);
    for _ in 0..start.0 {
        textarea.move_cursor(tui_textarea::CursorMove::Down);
    }
    textarea.move_cursor(tui_textarea::CursorMove::Head);
    for _ in 0..start.1 {
        textarea.move_cursor(tui_textarea::CursorMove::Forward);
    }

    // Select to end of block
    textarea.start_selection();
    textarea.move_cursor(tui_textarea::CursorMove::Top);
    for _ in 0..finish.0 {
        textarea.move_cursor(tui_textarea::CursorMove::Down);
    }
    textarea.move_cursor(tui_textarea::CursorMove::Head);
    for _ in 0..finish.1 {
        textarea.move_cursor(tui_textarea::CursorMove::Forward);
    }

    textarea.delete_next_char(); // deletes selection
}

pub fn copy_block(textarea: &mut TextArea, begin: (usize, usize), end: (usize, usize)) {
    let text = get_block_text(textarea, begin, end);
    textarea.insert_str(&text);
}

pub fn move_block(textarea: &mut TextArea, begin: (usize, usize), end: (usize, usize)) {
    let text = get_block_text(textarea, begin, end);
    delete_block(textarea, begin, end);
    textarea.insert_str(&text);
}

fn normalize_range(begin: (usize, usize), end: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    if begin.0 < end.0 || (begin.0 == end.0 && begin.1 <= end.1) {
        (begin, end)
    } else {
        (end, begin)
    }
}
