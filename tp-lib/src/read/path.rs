use anyhow::{Context, Result};
use std::{
    fs::{self, metadata},
    os::unix::fs::MetadataExt,
    path::Path,
};

/// Opens the configuration file and returns its content as a string.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the configuration file.
///
/// # Returns
///
/// * `Result<String, anyhow::Error>` - A Result containing the file content as a string if successful, or an error if the file cannot be opened.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened.
pub fn open_file_config(fileconfig: &Path) -> Result<String> {
    let metadata = metadata(fileconfig).context("Failed to get the metadata")?;

    // check if the file empty or not
    if metadata.size() == 0 {
        return Err(anyhow::anyhow!("File is empty fill it with something"));
    }

    if fileconfig.exists() {
        let content = fs::read_to_string(fileconfig).context("Failed to read file")?;
        Ok(content)
    } else {
        Err(anyhow::anyhow!(
            "File path does not exist or the file name is not right check again"
        ))
    }
}
