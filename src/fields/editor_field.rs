use crate::Config;
use serde::Deserialize;

/// General config of editor
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
impl EditorConfig {
    pub fn see_what_is_active(fileconfig: Config) {
        if fileconfig.editor.mouse {
            println!("The mouse is active")
        }
        if fileconfig.editor.autosave {
            println!("The auto save is active")
        }
        if fileconfig.editor.autoformat {
            println!("The auto format is active")
        }
        if !fileconfig.editor.mouse {
            println!("The mouse is inactive")
        }
        if !fileconfig.editor.autosave {
            println!("The auto save is inactive")
        }
        if !fileconfig.editor.autoformat {
            println!("The auto format is inactive")
        }
    }
}
