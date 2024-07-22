use crate::fields::editor_field::{Config, EditorConfig, StatuslineConfig};
use crate::{colprintln, eclprintln};
/// # Generate
///
/// This module provides functionality for generating a configuration file for the text editor.
///
/// The configuration file is generated in TOML format and contains various settings for the editor,
/// such as the theme, mouse support, auto-save and auto-format options, and status line configuration.
///
/// The `Generate` struct provides a single `new` method that generates the configuration file and
/// returns a handle to the file. If the file already exists, it is backed up and a new file is created.
///
/// ## Examples
///
/// To generate a configuration file, simply call the `new` method on the `Generate` struct:
///
/// ```
/// use my_text_editor::generate::Generate;
///
/// let file = Generate::new().expect("Failed to generate configuration file");
/// ```
///
/// This will generate a configuration file named `config.toml` in the current directory.
use anyhow::Context;
use std::fs::{rename, File};
use std::io::Write;
use std::path::Path;

/// Struct for generating a configuration file.
pub struct Generate;

impl Generate {
    /// Generates a configuration file and returns a handle to the file.
    ///
    /// The configuration file is generated in TOML format and contains various settings for the editor,
    /// such as the theme, mouse support, auto-save and auto-format options, and status line configuration.
    ///
    /// If the file already exists, it is backed up and a new file is created.
    ///
    /// # Returns
    ///
    /// A handle to the generated configuration file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration file cannot be created or written to.
    pub fn new() -> anyhow::Result<File, anyhow::Error> {
        colprintln!("<y>Generating the config... </y> \n");
        let filepath = Path::new("./config.toml");

        // Define config content
        let config_file = Config {
            theme: "gruvbox".to_string(),
            editor: EditorConfig {
                mouse: true,
                autosave: true,
                autoformat: true,
                statusline: StatuslineConfig {
                    separator: "|".to_string(),
                    left: vec!["mode".to_string(), "spinner".to_string()],
                },
            },
        };

        // Create file and write config to it
        let mut file = Self::create_file(filepath)?;
        let toml_string =
            toml::to_string(&config_file).context("Failed to serialize config to TOML")?;
        file.write_all(toml_string.as_bytes())
            .context("Failed to write config to file")?;

        colprintln!("<g>Config file created and written successfully</g>");
        Ok(file)
    }

    fn create_file(filename: &Path) -> anyhow::Result<File, anyhow::Error> {
        // Check if file already exists and remove it if it does
        if filename.exists() {
            let backed_up_file = format!("{}.bk", filename.display());
            eclprintln!(
                "<r>The file already exists, backing it up and creating a new one... \n</r>"
            );
            rename(filename, backed_up_file).context("couldn't rename")?;
        }

        // Create file
        File::create(filename).context("Failed to create file")
    }
}
