use ratatui::prelude::*;

#[derive(Debug, Clone)]
pub struct StyledSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}

pub fn highlight_markdown_line(line: &str) -> Vec<StyledSpan> {
    let mut spans = Vec::new();
    let trimmed = line.trim_start();

    // Heading detection
    if trimmed.starts_with("### ") {
        let offset = line.len() - trimmed.len();
        spans.push(StyledSpan {
            start: 0,
            end: offset + 4,
            style: Style::default().fg(Color::DarkGray),
        });
        spans.push(StyledSpan {
            start: offset + 4,
            end: line.len(),
            style: Style::default().fg(Color::Cyan),
        });
        return spans;
    } else if trimmed.starts_with("## ") {
        let offset = line.len() - trimmed.len();
        spans.push(StyledSpan {
            start: 0,
            end: offset + 3,
            style: Style::default().fg(Color::DarkGray),
        });
        spans.push(StyledSpan {
            start: offset + 3,
            end: line.len(),
            style: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        });
        return spans;
    } else if trimmed.starts_with("# ") {
        let offset = line.len() - trimmed.len();
        spans.push(StyledSpan {
            start: 0,
            end: offset + 2,
            style: Style::default().fg(Color::DarkGray),
        });
        spans.push(StyledSpan {
            start: offset + 2,
            end: line.len(),
            style: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        });
        return spans;
    }

    // Inline emphasis parsing
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        // Bold: **text**
        if i + 1 < len && bytes[i] == b'*' && bytes[i + 1] == b'*' {
            if let Some(end) = find_closing(line, i + 2, "**") {
                spans.push(StyledSpan {
                    start: i,
                    end: i + 2,
                    style: Style::default().fg(Color::DarkGray),
                });
                spans.push(StyledSpan {
                    start: i + 2,
                    end: end,
                    style: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                });
                spans.push(StyledSpan {
                    start: end,
                    end: end + 2,
                    style: Style::default().fg(Color::DarkGray),
                });
                i = end + 2;
                continue;
            }
        }

        // Italic: *text*
        if bytes[i] == b'*' && (i + 1 >= len || bytes[i + 1] != b'*') {
            if let Some(end) = find_closing_single(line, i + 1, '*') {
                spans.push(StyledSpan {
                    start: i,
                    end: i + 1,
                    style: Style::default().fg(Color::DarkGray),
                });
                spans.push(StyledSpan {
                    start: i + 1,
                    end: end,
                    style: Style::default().fg(Color::Green),
                });
                spans.push(StyledSpan {
                    start: end,
                    end: end + 1,
                    style: Style::default().fg(Color::DarkGray),
                });
                i = end + 1;
                continue;
            }
        }

        // Underline: ++text++
        if i + 1 < len && bytes[i] == b'+' && bytes[i + 1] == b'+' {
            if let Some(end) = find_closing(line, i + 2, "++") {
                spans.push(StyledSpan {
                    start: i,
                    end: i + 2,
                    style: Style::default().fg(Color::DarkGray),
                });
                spans.push(StyledSpan {
                    start: i + 2,
                    end: end,
                    style: Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED),
                });
                spans.push(StyledSpan {
                    start: end,
                    end: end + 2,
                    style: Style::default().fg(Color::DarkGray),
                });
                i = end + 2;
                continue;
            }
        }

        i += 1;
    }

    spans
}

fn find_closing(text: &str, start: usize, delimiter: &str) -> Option<usize> {
    text[start..].find(delimiter).map(|pos| start + pos)
}

fn find_closing_single(text: &str, start: usize, ch: char) -> Option<usize> {
    for (i, c) in text[start..].char_indices() {
        if c == ch {
            return Some(start + i);
        }
    }
    None
}

pub fn apply_syntax_highlighting(line: &str) -> Line<'static> {
    let styled_spans = highlight_markdown_line(line);

    if styled_spans.is_empty() {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(Color::White).bg(Color::Black),
        ));
    }

    let mut result_spans: Vec<Span<'static>> = Vec::new();
    let mut last_end = 0;

    for ss in &styled_spans {
        if ss.start > last_end {
            result_spans.push(Span::styled(
                line[last_end..ss.start].to_string(),
                Style::default().fg(Color::White).bg(Color::Black),
            ));
        }
        result_spans.push(Span::styled(
            line[ss.start..ss.end].to_string(),
            ss.style.bg(Color::Black),
        ));
        last_end = ss.end;
    }

    if last_end < line.len() {
        result_spans.push(Span::styled(
            line[last_end..].to_string(),
            Style::default().fg(Color::White).bg(Color::Black),
        ));
    }

    Line::from(result_spans)
}
