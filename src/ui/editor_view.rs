use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::app::App;
use crate::editor::buffer::EditorBuffer;

pub fn render(f: &mut Frame, area: Rect, app: &App, buffer: &mut EditorBuffer) {
    let (cursor_row, cursor_col) = buffer.textarea.cursor();
    let total_lines = buffer.textarea.lines().len();

    // Line number column width
    let lnum_width: u16 = if app.show_line_numbers {
        let digits = total_lines.max(1).to_string().len() as u16;
        digits + 1 // digits + trailing space
    } else {
        0
    };

    let text_x = area.x + lnum_width;
    let text_width = area.width.saturating_sub(lnum_width);
    let height = area.height as usize;

    // Update vertical scroll to keep cursor in view
    if cursor_row < buffer.scroll_row {
        buffer.scroll_row = cursor_row;
    } else if height > 0 && cursor_row >= buffer.scroll_row + height {
        buffer.scroll_row = cursor_row + 1 - height;
    }
    let scroll_row = buffer.scroll_row;

    // Render visible lines as plain text
    let visible: Vec<Line<'static>> = buffer.textarea.lines()
        .iter()
        .skip(scroll_row)
        .take(height)
        .map(|line| Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(Color::White).bg(Color::Black),
        )))
        .collect();

    let text_area = Rect { x: text_x, y: area.y, width: text_width, height: area.height };
    f.render_widget(
        Paragraph::new(visible).style(Style::default().bg(Color::Black)),
        text_area,
    );

    // Render line numbers
    if app.show_line_numbers {
        let lnum_area = Rect { x: area.x, y: area.y, width: lnum_width, height: area.height };
        let w = (lnum_width - 1) as usize;
        let lnum_lines: Vec<Line<'static>> = (scroll_row..scroll_row + height)
            .map(|i| {
                if i < total_lines {
                    Line::from(Span::styled(
                        format!("{:>w$} ", i + 1),
                        Style::default().fg(Color::DarkGray).bg(Color::Black),
                    ))
                } else {
                    Line::from(Span::raw(" ".repeat(lnum_width as usize)))
                }
            })
            .collect();
        f.render_widget(Paragraph::new(lnum_lines), lnum_area);
    }

    // Place cursor
    if cursor_row >= scroll_row {
        let cx = text_x + cursor_col as u16;
        let cy = area.y + (cursor_row - scroll_row) as u16;
        if cx < area.x + area.width && cy < area.y + area.height {
            f.set_cursor_position((cx, cy));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Position;
    use ratatui::Terminal;
    use tui_textarea::CursorMove;
    use crate::app::App;
    use crate::editor::buffer::EditorBuffer;

    /// Render `text` into a `width × height` terminal and return the backing buffer.
    fn render_to_buf(
        text: &str,
        width: u16,
        height: u16,
        show_line_numbers: bool,
    ) -> (ratatui::buffer::Buffer, EditorBuffer<'_>) {
        let mut app = App::new();
        app.show_line_numbers = show_line_numbers;
        let mut buffer = EditorBuffer::from_text(text);
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, f.area(), &app, &mut buffer)).unwrap();
        let buf = terminal.backend().buffer().clone();
        (buf, buffer)
    }

    /// Collect every cell in row `y` into a String.
    fn row(buf: &ratatui::buffer::Buffer, y: u16) -> String {
        (0..buf.area().width)
            .map(|x| {
                buf.cell(Position { x, y })
                    .map(|c| c.symbol().to_string())
                    .unwrap_or_default()
            })
            .collect()
    }

    // --- plain text ---

    #[test]
    fn test_plain_text_renders_as_is() {
        let (buf, _) = render_to_buf("hello world", 20, 3, false);
        assert!(row(&buf, 0).starts_with("hello world"));
    }

    #[test]
    fn test_empty_file_renders_blank_row() {
        let (buf, _) = render_to_buf("", 20, 3, false);
        assert_eq!(row(&buf, 0).trim(), "");
    }

    // --- markdown tokens are preserved verbatim ---

    #[test]
    fn test_italic_asterisk_tokens_not_stripped() {
        let (buf, _) = render_to_buf("*italic*", 20, 3, false);
        assert!(row(&buf, 0).starts_with("*italic*"));
    }

    #[test]
    fn test_italic_underscore_tokens_not_stripped() {
        let (buf, _) = render_to_buf("_italic_", 20, 3, false);
        assert!(row(&buf, 0).starts_with("_italic_"));
    }

    #[test]
    fn test_bold_tokens_preserved() {
        let (buf, _) = render_to_buf("**bold**", 20, 3, false);
        assert!(row(&buf, 0).starts_with("**bold**"));
    }

    #[test]
    fn test_h1_hash_preserved() {
        let (buf, _) = render_to_buf("# Heading", 20, 3, false);
        assert!(row(&buf, 0).starts_with("# Heading"));
    }

    #[test]
    fn test_h2_hash_preserved() {
        let (buf, _) = render_to_buf("## Section", 20, 3, false);
        assert!(row(&buf, 0).starts_with("## Section"));
    }

    #[test]
    fn test_h3_hash_preserved() {
        let (buf, _) = render_to_buf("### Sub", 20, 3, false);
        assert!(row(&buf, 0).starts_with("### Sub"));
    }

    // --- multi-line ---

    #[test]
    fn test_multiple_lines_fill_rows() {
        let (buf, _) = render_to_buf("alpha\nbeta\ngamma", 20, 5, false);
        assert!(row(&buf, 0).starts_with("alpha"));
        assert!(row(&buf, 1).starts_with("beta"));
        assert!(row(&buf, 2).starts_with("gamma"));
    }

    #[test]
    fn test_lines_beyond_height_are_clipped() {
        let (buf, _) = render_to_buf("a\nb\nc\nd\ne", 20, 3, false);
        assert!(row(&buf, 0).starts_with("a"));
        assert!(row(&buf, 1).starts_with("b"));
        assert!(row(&buf, 2).starts_with("c"));
        // rows d and e are outside the 3-line viewport
    }

    // --- scrolling ---

    #[test]
    fn test_scroll_row_zero_when_cursor_at_top() {
        let (_, buffer) = render_to_buf("a\nb\nc\nd", 20, 3, false);
        assert_eq!(buffer.scroll_row, 0);
    }

    #[test]
    fn test_scroll_advances_to_keep_cursor_visible() {
        let app = App::new();
        let mut buffer = EditorBuffer::from_text("a\nb\nc\nd\ne\nf");
        buffer.textarea.move_cursor(CursorMove::Bottom);

        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, f.area(), &app, &mut buffer)).unwrap();

        assert!(buffer.scroll_row > 0, "scroll_row should advance past 0");
        let buf = terminal.backend().buffer().clone();
        assert!(row(&buf, 2).starts_with("f"), "last line should be visible at bottom row");
    }

    // --- line numbers ---

    #[test]
    fn test_line_numbers_shown_when_enabled() {
        let (buf, _) = render_to_buf("hello", 20, 3, true);
        let r = row(&buf, 0);
        assert!(r.contains('1'), "line number should appear");
        assert!(r.contains("hello"), "text should follow line number");
    }

    #[test]
    fn test_text_starts_at_column_zero_without_line_numbers() {
        let (buf, _) = render_to_buf("hello", 20, 3, false);
        assert!(row(&buf, 0).starts_with("hello"));
    }

    #[test]
    fn test_line_number_width_grows_with_line_count() {
        // 10 lines needs 2-digit line numbers → width 3 (2 digits + space)
        let text = (1..=10).map(|i| format!("line{i}")).collect::<Vec<_>>().join("\n");
        let (buf, _) = render_to_buf(&text, 30, 10, true);
        // Row 9 (0-indexed) should show " 10 line10"
        let r = row(&buf, 9);
        assert!(r.contains("10"), "two-digit line number should appear");
    }
}
