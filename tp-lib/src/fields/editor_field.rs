//! # Config for the Text Editor
//!
//! This module represents the configuration for the text editor. Its aim is to validate the configuration of the `config.toml` within a specified path.

#![allow(dead_code)]

use crate::path::read::read_path_as_string;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use toml::Value;

/// Represents the configuration for the text editor.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Theme of the text editor. It could be `gruvbox`, `onedark`, or more. It is a string.
    pub theme: String,
    /// Editor configuration.
    pub editor: EditorConfig,
}

/// Represents the editor-specific configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct EditorConfig {
    /// Indicates if mouse navigation through the buffer is enabled. It could be `false` or `true`.
    pub mouse: bool,
    #[serde(rename = "auto-save")]
    /// Auto-save feature that saves automatically when you are away from your terminal.
    pub autosave: bool,
    #[serde(rename = "auto-format")]
    /// Formats text during saving.
    pub autoformat: bool,
    /// Status line configuration.
    pub statusline: StatuslineConfig,
}

/// Represents the status line configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct StatuslineConfig {
    /// Separator character that separates elements of the status line. It could be `|`.
    pub separator: String,
    /// Elements to display on the left of the status line.
    pub left: Vec<String>,
}

impl Config {
    /// Creates a new `Config` object from a file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the configuration file.
    ///
    /// # Returns
    ///
    /// * `Result<Value, anyhow::Error>` - A Result containing the configuration values as `toml::Value` if successful, or an error if parsing or validation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be opened or the contents cannot be parsed or validated.
    pub fn new(file_path: &Path) -> Result<Value, anyhow::Error> {
        let file_content = read_path_as_string(file_path)?;
        let config_values: toml::Value = file_content.parse().context("can't parse")?;
        match Self::validate(&config_values) {
            Ok(_) => Ok(config_values),
            Err(error) => anyhow::bail!("Error parsing config: {}", error),
        }
    }

    /// Validates the configuration values.
    ///
    /// # Arguments
    ///
    /// * `config_values` - A reference to the `toml::Value` containing the configuration values.
    ///
    /// # Returns
    ///
    /// * `Result<(), anyhow::Error>` - An empty Result if validation is successful, or an error if validation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the specified fields are not of the expected type.
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
