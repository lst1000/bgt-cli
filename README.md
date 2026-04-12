# bgt-cli

**bgt-cli** is a command-line tool for managing and tracking personal budgets using monthly TOML configuration files.

It calculates gross and net income, tax deductions, total expenses, and surplus. Configuration is separated by month (`YYYY-MM.toml`) and stored in the system config directory. You can create and edit configuration files directly via the command line.

## Features

- Reads per-month TOML configuration files from your system's config directory  
  (e.g. `~/.config/bgt-cli/2025-08.toml` on Linux or `~/Library/Application Support/bgt-cli/2025-08.toml` on macOS)
- Calculates gross pay, deductions, net pay, total expenses, and surplus
- Supports optional overtime breakdowns
- Create a new config file using `-c`, or edit with `-e`
- Clean terminal output, with aligned columns and currency formatting

## Usage

```bash
bgt-cli [-f YYYY-MM | -c YYYY-MM | -e YYYY-MM | -h]
```

## Options

- `-f YYYY-MM`  
  Load and display budget information for the given month.

- `-c YYYY-MM`  
  Create a new configuration file for the given month. If it already exists, an error is shown.

- `-e YYYY-MM`  
  Edit the configuration file for the given month using `$EDITOR`, or fall back to `nano`. If the file does not exist, it is created first.

- `-h`  
  Display the help message.

## Configuration

The program expects a TOML file named `YYYY-MM.toml` stored in your system’s config directory.  
Example paths:
- Linux: `~/.config/bgt-cli/<YYYY-MM>.toml`
- macOS: `~/Library/Application Support/bgt-cli/<YYYY-MM>.toml`
- Windows: `~\AppData\Roaming\bgt-cli\<YYYY-MM>.toml`

This file should include the following sections:

## Installation

Install using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install bgt-cli
```

## Licence

[MIT Licence](LICENSE).  

## Author

Written by [Laurence Stock-Tully](https://github.com/lst1000)
