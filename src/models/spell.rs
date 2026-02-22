#[derive(Debug, Clone)]
pub struct WordLocation {
    pub offset: usize,
    #[allow(dead_code)]
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct MisspelledWord {
    pub word: String,
    pub locations: Vec<WordLocation>,
    pub suggestions: Vec<String>,
}

impl MisspelledWord {
    pub fn occurrence_count(&self) -> usize {
        self.locations.len()
    }
}
