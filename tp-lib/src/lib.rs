/// methods for parsing the config file
pub mod fields {
    pub mod editor_field;
}

/// methods for identifying and reading the file confih
pub mod path {
    pub mod read;
}

/// methods for generating the config
pub mod generate {
    pub mod genrate_config;
}
#[macro_use]
pub mod macros;
