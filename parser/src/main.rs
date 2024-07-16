#[macro_use]
extern crate tparser;
use clap::Parser;
use std::path::PathBuf;
use tparser::read::path::open_file_config;

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
                if let Err(err) = open_file_config(file_config) {
                    eclprintln!("<r>{}</r>", err);
                    std::process::exit(1);
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
