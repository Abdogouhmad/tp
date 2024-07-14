use serde::Deserialize;
use std::fs;
use std::io::Error;
use toml::de;

mod fields;

use fields::editor_field::EditorConfig;

#[macro_use]
mod print_macro;

/// Define a struct to represent the configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    theme: String,
    editor: EditorConfig,
}

/// For status line

// Main function
fn main() -> Result<(), Error> {
    // Read the config file and parse it

    let config_file = read_config("example.toml")?;

    // Deserialize the configuration
    let config: Config = de::from_str(&config_file).expect("Failed to parse TOML");
    // println!("{:#?}", config);

    // Output the theme
    let theme = &config.theme;
    colprintln!("<m>Theme: </m> {}", theme);

    // sep
    let sep = &config.editor.statusline.separator;
    colprintln!("<r>Your seprator</r>: {}", sep);

    let left_arm = &config.editor.statusline.left;
    colprintln!("<c>Your letf arm</c>: {:#?}", left_arm);

    // things are activated with the boolens
    EditorConfig::see_what_is_active(config);
    Ok(())
}

// Function to read the config file
fn read_config(filename: &str) -> Result<String, Error> {
    let file_content = fs::read_to_string(filename)?;
    Ok(file_content)
}
