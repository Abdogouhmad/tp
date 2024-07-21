use crate::{colprintln, eclprintln};
use std::fs::File;
use std::path::Path;

pub struct Generate;

impl Generate {
    pub fn new() -> anyhow::Result<File, anyhow::Error> {
        colprintln!("<g> generating the config </g> \n");
        let filepath = Path::new("./config.toml");

        match Self::create_file(filepath) {
            Ok(file) => {
                colprintln!("<g>Config file created successfully</g>");
                Ok(file)
            }
            Err(e) => anyhow::bail!("{}", e),
        }
    }

    fn create_file(filename: &Path) -> anyhow::Result<File, anyhow::Error> {
        // check if the file is already there
        if filename.exists() {
            eclprintln!("<r> The file is already created</r>");
            std::process::exit(100)
        }

        let file_created = File::create(filename);

        match file_created {
            Ok(file) => Ok(file),
            Err(e) => anyhow::bail!("Sorry I couldn't create a file: {}", e),
        }
    }
}
