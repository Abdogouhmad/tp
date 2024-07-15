#[macro_use]
extern crate tparser;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Tp is a command line interface for generating and validating helix configuration.
#[derive(Parser, Debug)]
#[command(version = "24.7.14", long_about)]
pub struct Tp {
    /// Check the file config in a specific dir -c=path/ --check=path
    #[arg(short, long, value_name = "FILE CONFIG")]
    check: Option<PathBuf>,
    /// Generate an absolute config
    #[arg(short, long)]
    generate: bool,
}

fn main() {
    let arg = Tp::parse();

    if let Some(file_config) = arg.check.as_deref() {
        match file_config.to_str() {
            Some(conf_str) if get_extension_from_filename(conf_str) == Some("toml") => {
                match fs::read_to_string(file_config) {
                    Ok(contents) => colprintln!("<g>File contents:</g>\n{}", contents),
                    Err(err) => eprintln!("Failed to read file: {}", err),
                }
            }
            _ => {
                eclprintln!("<r>This file is not a toml file or invalid path</r>");
                std::process::exit(1);
            }
        }
    }

    if arg.generate {
        colprintln!("<g>Generating the config file in ~/.config/helix ... </g>");
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}
