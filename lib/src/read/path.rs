use anyhow::{Context, Result};
use std::{
    fs::{self, metadata},
    os::unix::fs::MetadataExt,
    path::Path,
};

/// # open_file_config
/// get the path of file, read it and return its content  
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
