# TP

TP is a Rust command line tool designed to parse and check configuration files for text editors. It currently supports the following text editors:

- [Helix](https://github.com/helix-editor/helix/)

## Features

TP offers a range of features to assist users in managing their text editor configurations:

- **Validation:** Checks the configuration files for errors and inconsistencies.
- **Suggestions:** Provides helpful suggestions to facilitate the correction of configuration issues.
- **Generation:** Automatically generates configuration files, ideal for users who are new to creating them or prefer to avoid manual setup.

## Usage

### Installation

You can install TP using Cargo, Rust's package manager. Run the following command in your terminal:

```sh
cargo install --git=https://github.com/Abdogouhmad/tp
```

### Commands

TP provides various commands to interact with your configuration files:

#### Check

To check a configuration file, use the `check` command followed by the path to your configuration file:

```bash
tp -c path/to/config.toml
# or
tp --check path/to/config.toml
```

#### Generate

To generate a new configuration file, use the `generate` command:

```bash
tp --generate
# or
tp -g
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or find a bug, please open an issue or submit a pull request.
