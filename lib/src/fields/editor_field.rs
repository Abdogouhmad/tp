#![allow(dead_code)]
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use toml::Value;
// use toml::de::Error as TomlError;

use crate::read::path::open_file_config;

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
    pub fn new(file_path: &Path) -> Result<Value, anyhow::Error> {
        let file_content = open_file_config(file_path)?;
        let config_values: toml::Value = file_content.parse().context("can't parse")?;
        match Self::validate(&config_values) {
            Ok(_) => Ok(config_values),
            Err(error) => anyhow::bail!("Error parsing config: {}", error),
        }
    }

    // match me
    fn validate(config_values: &Value) -> Result<(), anyhow::Error> {
        if let Some(editor) = config_values.get("editor") {
            if let Some(mouse) = editor.get("mouse") {
                if !mouse.is_bool() {
                    anyhow::bail!(
                        "Invalid type for `mouse`: expected either true or false as boolean, found {:?}",
                        mouse
                    );
                }
            }
            if let Some(autosave) = editor.get("auto-save") {
                if !autosave.is_bool() {
                    anyhow::bail!(
                        "Invalid type for `auto-save`: expected either true or false as boolean, found {:?}",
                        autosave
                    );
                }
            }
            if let Some(autoformat) = editor.get("auto-format") {
                if !autoformat.is_bool() {
                    anyhow::bail!(
                        "Invalid type for `auto-format`: expected either true or false as boolean, found {:?}",
                        autoformat
                    );
                }
            }
        }

        Ok(())
    }
}
