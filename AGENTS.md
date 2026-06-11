# bgt-cli Development Guide

## Project Overview
Single-file Rust CLI tool for managing personal budgets using monthly TOML config files.
- Reads/writes `YYYY-MM.toml` files from system config directory
- Calculates gross pay, UK tax deductions, net pay, and expenses
- Displays formatted terminal output with currency formatting

## Quick Start

### Build
```bash
cargo build --release
```

### Install
```bash
cargo install bgt-cli
```

### Run
```bash
bgt-cli -f YYYY-MM   # View budget
bgt-cli -c YYYY-MM   # Create new config
bgt-cli -e YYYY-MM   # Edit config
```

## Architecture

### Entry Point
- **`src/main.rs`** - Single binary, no Cargo.toml dev/build profiles

### Structure
```
src/
  main.rs          # CLI entry point, config parsing, tax calculations
example.toml       # Template config
target/            # Build artifacts (.gitignore'd)
```

### Dependencies
- `dirs` - Config directory path
- `toml` / `serde` - Config parsing
- `regex` - Filename validation (YYYY-MM format)
- `uk-tax` - UK income tax & NI calculations

## Configuration

### File Location
- **Linux**: `~/.config/bgt-cli/2025-08.toml`
- **macOS**: `~/Library/Application Support/bgt-cli/2025-08.toml`
- **Windows**: `~\AppData\Roaming\bgt-cli\2025-08.toml`

### Valid Months
```text
2025-01, 2025-02, ..., 2025-12
```
Validation uses regex `^\d{4}-(0[1-9]|1[0-2])$` - invalid months rejected.

### TOML Format
```toml
[Income]
Salary = 3000
Allowance = 0
Bonus = 100
[Income.Overtime]  # Optional: "1.5" = 5, "2.0" = 3
[Expenses]
Rent = 800
[Tax]
Year = 2025         # 2025 = 2025/26 tax year
# Personal_Allowance = 12570  # Optional override
```

### Tax Year Mapping
- Config `Year = 2025` → Uses `uk-tax::tax_years::TAX_YEAR_2025`
- Valid years: 2015-2026 (hardcoded in `tax_year()` function)
- **Error**: Years outside 2015-2026 cause panic

## Implementation Details

### CLI Arguments
```rust
bgt-cli [OPTIONS] [arg1] [fname]
```
- `arg1` in `{"-f", "-c", "-e", "-h"}`, `fname` in `YYYY-MM`

### Config Directory Discovery
Uses `dirs::config_dir()` to locate platform-specific directory.

### Edit Fallback
- If `$EDITOR` not set, uses `nano` (non-Windows) or `notepad.exe` (Windows).

### Calculation Order
1. Overtime: `hourly_pay × hours × overtime_rate`
2. Gross pay: Salary + Allowance + Overtime + Bonus + Misc + Pension
3. Annual gross: Gross × 12
4. Tax: `uk-tax::income_tax()` and `uk-tax::national_ins()` / 12
5. Net pay: Gross pay − (Tax + NI)
6. Surplus: Net pay − Total expenses

## Known Constraints
- No test suite defined (no `[[test]]` in Cargo.toml)
- `uk-tax` crate provides tax calculations - no local tax logic
- Hardcoded tax years; unsupported year panics
- Single binary, no workspace packages

## Common Pitfalls
1. Invalid month format (`2025-13`) → CLI error
2. Year outside 2015-2026 → panic
3. Config file path typo → "Not Found" error
4. Month already exists with `-c` → "Budget File Exists" error
5. Running without arguments → "No Arguments Specified"

## Editor Integration
```bash
EDITOR=vim bgt-cli -e YYYY-MM   # Override default editor
```

## Git Ignore
Only `/target` is ignored. No other build artifacts or configs excluded.
