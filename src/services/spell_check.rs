use std::collections::HashSet;
use std::fs;

use levenshtein::levenshtein;

use crate::models::spell::{MisspelledWord, WordLocation};
use crate::services::word_extractor::extract_words;

/// Embedded dictionary - loaded at compile time
static BUILTIN_DICT: &str = include_str!("../../resources/english-words.txt");

pub struct SpellChecker {
    dictionary: HashSet<String>,
    user_dictionary: HashSet<String>,
}

impl SpellChecker {
    pub fn new() -> Self {
        let dictionary = load_dict_from_str(BUILTIN_DICT);
        let user_dictionary = load_user_dict();
        Self { dictionary, user_dictionary }
    }

    pub fn is_correct(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.dictionary.contains(&lower) || self.user_dictionary.contains(&lower)
    }

    pub fn suggestions(&self, word: &str, max: usize) -> Vec<String> {
        let lower = word.to_lowercase();
        let mut candidates: Vec<(usize, &String)> = self
            .dictionary
            .iter()
            .filter(|w| {
                let dist = levenshtein(&lower, w);
                dist <= 2 && (w.len() as i32 - lower.len() as i32).abs() <= 3
            })
            .map(|w| (levenshtein(&lower, w), w))
            .collect();

        candidates.sort_by_key(|(d, _)| *d);
        candidates.truncate(max);
        candidates.into_iter().map(|(_, w)| w.clone()).collect()
    }

    pub fn check_text(&self, text: &str) -> Vec<MisspelledWord> {
        let words = extract_words(text);
        let mut misspelled: std::collections::HashMap<String, Vec<WordLocation>> =
            std::collections::HashMap::new();

        for (offset, word) in &words {
            if !self.is_correct(word) {
                let entry = misspelled
                    .entry(word.to_lowercase())
                    .or_default();
                entry.push(WordLocation {
                    offset: *offset,
                    length: word.len(),
                });
            }
        }

        let mut result: Vec<MisspelledWord> = misspelled
            .into_iter()
            .map(|(word, locations)| {
                let suggestions = self.suggestions(&word, 8);
                MisspelledWord { word, locations, suggestions }
            })
            .collect();

        result.sort_by(|a, b| {
            a.locations
                .first()
                .map(|l| l.offset)
                .cmp(&b.locations.first().map(|l| l.offset))
        });
        result
    }

    pub fn add_to_user_dictionary(&mut self, word: &str) -> std::io::Result<()> {
        let lower = word.to_lowercase();
        self.user_dictionary.insert(lower.clone());

        if let Some(path) = user_dict_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut existing = if path.exists() {
                fs::read_to_string(&path)?
            } else {
                String::new()
            };
            existing.push('\n');
            existing.push_str(&lower);
            fs::write(path, existing)?;
        }
        Ok(())
    }
}

fn load_dict_from_str(s: &str) -> HashSet<String> {
    s.lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty() && l.len() >= 2)
        .collect()
}

fn load_user_dict() -> HashSet<String> {
    if let Some(path) = user_dict_path() {
        if let Ok(content) = fs::read_to_string(path) {
            return load_dict_from_str(&content);
        }
    }
    HashSet::new()
}

fn user_dict_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".vibe").join("user-dictionary.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_check() {
        let checker = SpellChecker::new();
        assert!(checker.is_correct("the"));
        assert!(checker.is_correct("hello"));
        assert!(!checker.is_correct("teh"));
    }

    #[test]
    fn test_suggestions() {
        let checker = SpellChecker::new();
        let suggestions = checker.suggestions("teh", 5);
        assert!(suggestions.contains(&"the".to_string()) || !suggestions.is_empty());
    }
}
