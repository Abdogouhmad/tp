use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use toml::de::Error as TomlError;

use crate::{colprintln, read::path::open_file_config};

// Assuming colprintln! is a macro defined elsewhere in your project
// If not, you'll need to implement it or replace it with println!

#[derive(Debug, Deserialize)]
pub struct Config {
    theme: String,
    editor: EditorConfig,
}

#[derive(Debug, Deserialize)]
pub struct EditorConfig {
    mouse: bool,
    #[serde(rename = "auto-save")]
    autosave: bool,
    #[serde(rename = "auto-format")]
    autoformat: bool,
    pub statusline: StatuslineConfig,
}

#[derive(Debug, Deserialize)]
pub struct StatuslineConfig {
    pub separator: String,
    pub left: Vec<String>,
}

impl Config {
    pub fn new(file_path: &Path) -> Result<Self> {
        let file_content = open_file_config(file_path)?;

        match toml::from_str(&file_content) {
            Ok(config) => Ok(config),
            Err(e) => {
                let suggestion = check_error(&e);
                colprintln!("<y>Suggestion:</y> {} \n\n", suggestion);
                Err(e.into())
            }
        }
    }

    pub fn see_what_is_active(&self) {
        colprintln!("<g>Captured Configuration at The moment </g>");
        println!(
            "Theme: {}\nMouse: {}\nAuto Save: {}\nAuto Format: {}",
            self.theme,
            if self.editor.mouse {
                "active"
            } else {
                "inactive"
            },
            if self.editor.autosave {
                "active"
            } else {
                "inactive"
            },
            if self.editor.autoformat {
                "active"
            } else {
                "inactive"
            }
        );
    }
}

pub fn check_error(error: &TomlError) -> String {
    let error_message = error.to_string();

    if let Some((line, column, field)) = extract_field_from_error(&error_message) {
        match field {
            "mouse" => format!("Error at line {}, column {}. For the 'mouse' field, please choose between true or false.", line, column),
            "auto-save" => format!("Error at line {}, column {}. For the 'auto-save' field, please choose between true or false.", line, column),
            "auto-format" => format!("Error at line {}, column {}. For the 'auto-format' field, please choose between true or false.", line, column),
            "theme" => format!("Error at line {}, column {}. For the 'theme' field, please provide a valid string.", line, column),
            "separator" => format!("Error at line {}, column {}. For the 'separator' field, please provide a valid string.", line, column),
            "left" => format!("Error at line {}, column {}. For the 'left' field, please provide a valid array of strings.", line, column),
            _ => format!("Error at line {}, column {}. An error occurred in the '{}' field. Please check its value.", line, column, field), // what provides
        }
    } else {
        format!(
            "An error occurred while parsing the TOML file: {}. Please check your configuration.",
            error_message
        )
    }
}

fn extract_field_from_error(error_message: &str) -> Option<(u32, u32, &str)> {
    let lines: Vec<&str> = error_message.lines().collect();
    if lines.len() < 4 {
        return None;
    }

    // Extract line and column
    let location_info = lines[0].trim();
    let location_parts: Vec<&str> = location_info.split(',').collect();
    if location_parts.len() != 2 {
        return None;
    }
    let line: u32 = location_parts[0]
        .trim()
        .strip_prefix("TOML parse error at line ")?
        .parse()
        .ok()?;
    let column: u32 = location_parts[1]
        .trim()
        .strip_prefix("column ")?
        .parse()
        .ok()?;

    // Extract field name
    let field_line = lines[2].trim();
    let field_parts: Vec<&str> = field_line.splitn(3, ' ').collect();
    if field_parts.len() < 2 {
        return None;
    }
    let field = field_parts[0];

    Some((line, column, field))
}
