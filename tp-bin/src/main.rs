#[macro_use]
extern crate tparser;

use clap::builder::{styling::AnsiColor, Styles};
use clap::Parser;
use std::fs::remove_file;
use std::path::PathBuf;
use tparser::{fields::editor_field::Config, generate::genrate_config::Generate};
/// Tp is a command line interface for generating and validating helix configuration.
#[derive(Parser, Debug)]
#[command(version = "24.7.14", about ,long_about, styles=style_docs())]
pub struct Tp {
    /// Check the file config in a specific path
    #[arg(short, long, value_name = "FILE CONFIG")]
    check: Option<PathBuf>,
    /// Generate an absolute config
    #[arg(short, long)]
    generate: bool,

    /// clean the back up within the current directory
    #[arg(long)]
    clean: bool,
}

fn main() {
    let arg = Tp::parse();

    if let Some(file_config) = arg.check.as_deref() {
        match file_config.to_str() {
            Some(conf_str) if get_extension_from_filename(conf_str) == Some("toml") => {
                match Config::new(file_config) {
                    Ok(_) => {
                        colprintln!("<g>Config file loaded successfully, and everything is ok</g>");
                    }
                    Err(err) => {
                        eclprintln!("<r>Error: {}</r>", err);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eclprintln!("<r>This file is not a toml file or invalid path</r>");
                std::process::exit(1);
            }
        }
    }

    if arg.generate {
        match Generate::generate_config() {
            Ok(_) => {}
            Err(e) => {
                eclprintln!("<r>Error: {}</r>", e);
                std::process::exit(1);
            }
        }
    }

    // clean option cmd
    if arg.clean {
        match remove_file("config.toml.bk") {
            Ok(_) => colprintln!("<g> Config back up file is removed </g>"),
            Err(e) => eclprintln!("<r>Something went wrong:</r> {}", e),
        }
    }
}

// get the extension from file
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}

// style help of cli
fn style_docs() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightBlue.on_default())
        .header(AnsiColor::BrightYellow.on_default())
        .literal(AnsiColor::BrightMagenta.on_default())
        .invalid(AnsiColor::BrightRed.on_default())
        .error(AnsiColor::BrightRed.on_default())
        .valid(AnsiColor::BrightWhite.on_default())
        .placeholder(AnsiColor::BrightBlue.on_default())
}
