use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub editor: EditorConfig,
    pub ui: UiConfig,
    pub spell: SpellConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub word_wrap: bool,
    pub wrap_column: u16,
    pub tab_width: u8,
    pub show_line_numbers: bool,
    pub show_ruler: bool,
    pub insert_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub show_help_menu: bool,
    pub color_scheme: String,
    pub wordstar_diamond: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellConfig {
    pub dictionary_path: String,
    pub user_dictionary_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig {
                word_wrap: true,
                wrap_column: 80,
                tab_width: 4,
                show_line_numbers: false,
                show_ruler: true,
                insert_mode: true,
            },
            ui: UiConfig {
                show_help_menu: true,
                color_scheme: "classic".to_string(),
                wordstar_diamond: false,
            },
            spell: SpellConfig {
                dictionary_path: String::new(),
                user_dictionary_path: String::new(),
            },
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Self {
        let config_path = path
            .map(|p| p.to_path_buf())
            .or_else(default_config_path);

        if let Some(path) = config_path {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }
}

fn default_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".vibe").join("config.toml"))
}
