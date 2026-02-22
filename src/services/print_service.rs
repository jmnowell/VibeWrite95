use std::io;
use std::path::Path;

use printpdf::*;

/// Export markdown content to a PDF file.
pub fn export_pdf(content: &str, output_path: &Path) -> io::Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("VibeWrite95 Export", Mm(215.9), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let margin_mm: f32 = 25.4; // 1 inch
    let start_y: f32 = 279.4 - margin_mm;
    let line_height: f32 = 6.0;
    let body_size: f32 = 12.0;

    let mut y = start_y;

    for line in content.lines() {
        let trimmed = line.trim_start();

        if y < margin_mm + line_height {
            let (new_page, new_layer) = doc.add_page(Mm(215.9), Mm(279.4), "Layer 1");
            let _ = doc.get_page(new_page).get_layer(new_layer);
            y = start_y;
        }

        if trimmed.starts_with("# ") {
            let text = trimmed.strip_prefix("# ").unwrap_or(trimmed);
            let size = 24.0_f32;
            current_layer.use_text(text, size, Mm(margin_mm), Mm(y), &font_bold);
            y -= line_height * (size / body_size) + line_height * 0.5;
        } else if trimmed.starts_with("## ") {
            let text = trimmed.strip_prefix("## ").unwrap_or(trimmed);
            let size = 18.0_f32;
            current_layer.use_text(text, size, Mm(margin_mm), Mm(y), &font_bold);
            y -= line_height * (size / body_size) + line_height * 0.5;
        } else if trimmed.starts_with("### ") {
            let text = trimmed.strip_prefix("### ").unwrap_or(trimmed);
            let size = 14.0_f32;
            current_layer.use_text(text, size, Mm(margin_mm), Mm(y), &font_bold);
            y -= line_height * (size / body_size) + line_height * 0.5;
        } else {
            // Render body text with inline bold support
            let runs = parse_inline_runs(line);
            let mut x = margin_mm;
            for run in &runs {
                if !run.text.is_empty() {
                    let f = if run.bold { &font_bold } else { &font };
                    current_layer.use_text(&run.text, body_size, Mm(x), Mm(y), f);
                    x += text_width_mm(&run.text, body_size);
                }
            }
            y -= line_height;
        }
    }

    let bytes = doc.save_to_bytes()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    std::fs::write(output_path, bytes)
}

/// A run of text with a single style.
struct TextRun {
    text: String,
    bold: bool,
}

/// Parse a line into runs of plain and bold text by handling `**...**` markers.
/// Italic (`*...*`) and underline (`++...++`) markers are stripped but rendered plain,
/// since we only have regular and bold builtin fonts.
fn parse_inline_runs(line: &str) -> Vec<TextRun> {
    let mut runs: Vec<TextRun> = Vec::new();
    let mut current = String::new();
    let mut bold = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            // Bold delimiter **
            chars.next();
            // Flush current run
            if !current.is_empty() {
                runs.push(TextRun { text: current.clone(), bold });
                current.clear();
            }
            bold = !bold;
        } else if ch == '*' {
            // Italic * — strip marker, keep same bold state
        } else if ch == '+' && chars.peek() == Some(&'+') {
            // Underline ++ — strip marker
            chars.next();
        } else if ch == '_' && chars.peek() == Some(&'_') {
            // Bold via __ syntax
            chars.next();
            if !current.is_empty() {
                runs.push(TextRun { text: current.clone(), bold });
                current.clear();
            }
            bold = !bold;
        } else if ch == '_' {
            // Italic _ — strip marker
        } else if ch == '`' {
            // Inline code — strip backtick
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        runs.push(TextRun { text: current, bold });
    }

    runs
}

/// Approximate Helvetica character width in mm at a given point size.
/// Uses simplified AFM metrics (widths per 1000 units, converted to mm).
fn char_width_mm(ch: char, font_size: f32) -> f32 {
    let units: f32 = match ch {
        ' ' => 278.0,
        '!' | ',' | '.' | ':' | ';' | '\'' | '`' => 278.0,
        'I' | '|' => 278.0,
        'f' | 'i' | 'j' | 'l' | 'r' | 't' => 333.0,
        '(' | ')' | '[' | ']' => 333.0,
        '"' | '-' => 333.0,
        'm' | 'w' => 722.0,
        'W' | 'M' => 778.0,
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L'
        | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'X' | 'Y' | 'Z' => 667.0,
        _ => 556.0,
    };
    // width_mm = (units / 1000) * font_size_pts * (1pt in mm = 0.352778)
    (units / 1000.0) * font_size * 0.352778
}

fn text_width_mm(text: &str, font_size: f32) -> f32 {
    text.chars().map(|c| char_width_mm(c, font_size)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_inline_runs ---

    #[test]
    fn test_plain_text_is_single_run() {
        let runs = parse_inline_runs("This is a test.");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].text, "This is a test.");
        assert!(!runs[0].bold);
    }

    #[test]
    fn test_bold_only_run() {
        let runs = parse_inline_runs("**This is another test.**");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].text, "This is another test.");
        assert!(runs[0].bold);
    }

    #[test]
    fn test_mixed_plain_and_bold() {
        let runs = parse_inline_runs("Some **bold** words here.");
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[0].text, "Some ");
        assert!(!runs[0].bold);
        assert_eq!(runs[1].text, "bold");
        assert!(runs[1].bold);
        assert_eq!(runs[2].text, " words here.");
        assert!(!runs[2].bold);
    }

    #[test]
    fn test_italic_markers_stripped_as_plain() {
        let runs = parse_inline_runs("*italic text*");
        // Markers stripped, content rendered as plain
        let combined: String = runs.iter().map(|r| r.text.as_str()).collect();
        assert_eq!(combined, "italic text");
        assert!(runs.iter().all(|r| !r.bold));
    }

    #[test]
    fn test_double_underscore_bold() {
        let runs = parse_inline_runs("__bold via underscores__");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].text, "bold via underscores");
        assert!(runs[0].bold);
    }

    #[test]
    fn test_empty_line_produces_no_runs() {
        let runs = parse_inline_runs("");
        assert!(runs.is_empty());
    }

    // --- heading prefix stripping (mirrors export_pdf logic) ---

    #[test]
    fn test_h1_prefix_stripped() {
        let line = "# This is a test.";
        let trimmed = line.trim_start();
        assert!(trimmed.starts_with("# "));
        let text = trimmed.strip_prefix("# ").unwrap();
        assert_eq!(text, "This is a test.");
    }

    #[test]
    fn test_h2_prefix_stripped() {
        let line = "## Section";
        let trimmed = line.trim_start();
        let text = trimmed.strip_prefix("## ").unwrap();
        assert_eq!(text, "Section");
    }

    #[test]
    fn test_h3_prefix_stripped() {
        let line = "### Subsection";
        let trimmed = line.trim_start();
        let text = trimmed.strip_prefix("### ").unwrap();
        assert_eq!(text, "Subsection");
    }

    // --- text_width_mm ---

    #[test]
    fn test_text_width_is_positive() {
        assert!(text_width_mm("Hello", 12.0) > 0.0);
    }

    #[test]
    fn test_longer_text_is_wider() {
        let short = text_width_mm("Hi", 12.0);
        let long = text_width_mm("Hello world", 12.0);
        assert!(long > short);
    }

    #[test]
    fn test_larger_font_is_wider() {
        let small = text_width_mm("test", 12.0);
        let large = text_width_mm("test", 24.0);
        assert!(large > small);
    }
}
