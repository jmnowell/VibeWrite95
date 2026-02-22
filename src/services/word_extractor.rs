/// Extracts plain text words from markdown, skipping code blocks, URLs, etc.
pub fn extract_words(text: &str) -> Vec<(usize, String)> {
    let mut words = Vec::new();
    let mut in_code_block = false;
    let mut pos = 0;

    for line in text.lines() {
        // Track fenced code blocks
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            pos += line.len() + 1;
            continue;
        }

        if !in_code_block {
            let line_words = extract_words_from_line(line, pos);
            words.extend(line_words);
        }

        pos += line.len() + 1;
    }

    words
}

fn extract_words_from_line(line: &str, line_offset: usize) -> Vec<(usize, String)> {
    let mut words = Vec::new();
    let mut in_inline_code = false;
    let mut word_start: Option<usize> = None;
    let mut word_buf = String::new();
    let chars: Vec<(usize, char)> = line.char_indices().collect();

    let mut i = 0;
    while i < chars.len() {
        let (byte_pos, ch) = chars[i];

        // Toggle inline code
        if ch == '`' {
            if let Some(start) = word_start {
                if !word_buf.is_empty() {
                    words.push((line_offset + start, word_buf.clone()));
                }
                word_start = None;
                word_buf.clear();
            }
            in_inline_code = !in_inline_code;
            i += 1;
            continue;
        }

        if in_inline_code {
            i += 1;
            continue;
        }

        // Skip URLs in markdown links [text](url) - skip inside ()
        if ch == '(' && i > 0 && chars[i - 1].1 == ']' {
            // Skip until closing )
            while i < chars.len() && chars[i].1 != ')' {
                i += 1;
            }
            i += 1;
            continue;
        }

        // Skip markdown syntax chars at start: # * _ `
        if is_word_char(ch) {
            if word_start.is_none() {
                word_start = Some(byte_pos);
            }
            word_buf.push(ch);
        } else if ch == '\'' && word_start.is_some() {
            // Allow apostrophes within words (contractions)
            word_buf.push(ch);
        } else {
            if let Some(start) = word_start {
                // Trim trailing apostrophe
                let trimmed = word_buf.trim_end_matches('\'');
                if trimmed.len() >= 2 {
                    words.push((line_offset + start, trimmed.to_string()));
                }
                word_start = None;
                word_buf.clear();
            }
        }

        i += 1;
    }

    // Flush last word
    if let Some(start) = word_start {
        let trimmed = word_buf.trim_end_matches('\'');
        if trimmed.len() >= 2 {
            words.push((line_offset + start, trimmed.to_string()));
        }
    }

    words
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphabetic() || ch == '-'
}
