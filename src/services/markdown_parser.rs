use crate::models::outline_item::OutlineItem;

pub fn extract_outline(text: &str, source_file: Option<&str>) -> Vec<OutlineItem> {
    let mut items = Vec::new();

    for (line_number, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        let level = if trimmed.starts_with("### ") {
            Some(3)
        } else if trimmed.starts_with("## ") {
            Some(2)
        } else if trimmed.starts_with("# ") {
            Some(1)
        } else {
            None
        };

        if let Some(level) = level {
            let hashes = "#".repeat(level as usize);
            let title = trimmed.strip_prefix(&hashes)
                .unwrap_or(trimmed)
                .trim()
                .to_string();
            items.push(OutlineItem {
                title,
                level,
                line_number,
                source_file_path: source_file.map(|s| s.to_string()),
            });
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_outline() {
        let text = "# Hello\n\nSome text\n\n## World\n\n### Sub\n";
        let items = extract_outline(text, None);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].title, "Hello");
        assert_eq!(items[0].level, 1);
        assert_eq!(items[1].title, "World");
        assert_eq!(items[1].level, 2);
        assert_eq!(items[2].title, "Sub");
        assert_eq!(items[2].level, 3);
    }
}
