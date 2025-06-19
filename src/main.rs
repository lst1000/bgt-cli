use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use regex::Regex;
use serde::Deserialize;
use indexmap::IndexMap;

const EXAMPLE_TOML: &str = include_str!("../example.toml");

#[derive(Debug, Deserialize)]
struct Budget {
    #[serde(rename = "Income")]
    income: Income,
    #[serde(rename = "Expenses")]
    expenses: IndexMap<String, f64>,
    #[serde(rename = "Tax")]
    tax: Tax,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Income {
    salary: f64,
    allowance: f64,
    bonus: f64,
    misc: f64,
    pension: f64,
    overtime: Option<HashMap<String, f64>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Tax {
    #[serde(rename = "BR_Threshold")]
    br_threshold: f64,
    #[serde(rename = "BR_Rate")]
    br_rate: f64,
    #[serde(rename = "HR_Threshold")]
    hr_threshold: f64,
    #[serde(rename = "HR_Rate")]
    hr_rate: f64,
    #[serde(rename = "P_Threshold")]
    p_threshold: f64,
    #[serde(rename = "P_Rate")]
    p_rate: f64,
    #[serde(rename = "U_Threshold")]
    u_threshold: f64,
    #[serde(rename = "U_Rate")]
    u_rate: f64,
}

fn edit_config(config_path: &std::path::Path) {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let status = Command::new(editor)
        .arg(config_path)
        .status()
        .expect("Failed to Open Editor");
    
    if !status.success() {
        eprintln!("Failed to Edit the Budget File");
    }
}

fn create_config(config_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().expect("Configuration Directory Not Found").join("bgt-cli");
    let config_file = config_dir.join(format!("{}", config_path.to_string_lossy()));

    fs::create_dir_all(&config_dir)?;
    
    if !config_file.exists() {
        fs::write(&config_file, EXAMPLE_TOML)?;
        println!("\x1b[1mCreated config at {}\x1b[0m", config_file.display());
        edit_config(config_path);
    }
    std::process::exit(0);
}

fn config_search(fname: &str) -> PathBuf {
    let filename = format!("{}.toml", fname);
    let config_dir = dirs::config_dir().expect("Configuration Directory Not Found");

    config_dir.join("bgt-cli").join(filename)
}

fn print_help() {
    println!(
        "bgt-cli v0.1.1\nUsage: bgt-cli [OPTIONS]

Options:
    -f YYYY-MM          Print the the specified budget 
    -c YYYY-MM          Create a budget file
    -e YYYY-MM          Edit the specified budget
    -h                  Show this help message

Configuration File:
    This program looks for the configuration file in:
        macOS: ~/Library/Application Support/bgt-cli/<YYYY-MM>.toml
        Linux: ~/.config/bgt-cli/<YYYY-MM>.toml"
    );
}

fn print_error(message: &str) {
    eprintln!("Error: {}", message);
    eprintln!("Use '-h' to see available options");
    std::process::exit(1);
}

fn tax(gross: f64, b_thold: f64, b_rate: f64, h_thold: f64, h_rate: f64) -> f64 {
    if gross > h_thold {
        let h_amt = gross - h_thold;
        let b_amt = h_thold - b_thold;
        return (h_amt * h_rate) + (b_amt * b_rate);
    } else if gross > b_thold {
        return (gross - b_thold) * b_rate;
    }
    0.0
}

fn validate_fname(fname: &str) -> Result<(), String> {
    let rx = Regex::new(r"^\d{4}-(0[1-9]|1[0-2])$").unwrap();
    if rx.is_match(fname) {
        Ok(())
    } else {
       Err(format!("Invalid File Name: '{}' - Please Use: YYYY-MM", fname))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_error(&format!("No Arguments Specified"));
    }

    if args.len() == 2 && args[1] == "-h" {
        print_help();
        return Ok(());
    }

    if args.len() == 2 && args[1] != "-h" {
        print_error(&format!("Invalid Argument '{}'", args[1]));
    }
    
    let config_path = config_search(&args[2]);

    if args.len() > 2 {
        let arg = &args[1];
        let fname = &args[2];

        if arg == "-f" {
            if !config_path.exists() {
                print_error(&format!("Budget File Not Found: {}", config_path.display()));
            }
        } else if arg == "-e" {
            if let Err(e) = validate_fname(fname) {
                print_error(&e);
            }

            if !config_path.exists() {
                print_error(&format!("Budget File Not Found: {}", config_path.display()));
            }

            edit_config(&config_path);

            return Ok(());
        } else if arg == "-c" {
            if let Err(e) = validate_fname(fname) {
                print_error(&e);
            }

            if config_path.exists() {
                print_error(&format!("Budget File Exists: {}", config_path.display()));
            }

            create_config(&config_path)?;

            return Ok(());
        } else {
            print_error(&format!("Invalid Argument '{}'", arg));
        }
    }

    let content = fs::read_to_string(config_path)?;
    let budget: Budget = toml::from_str(&content)?;

    let hourly_pay = budget.income.salary / (1820.0 / 12.0);

    let mut overtime_pay = 0.0;

    if let Some(overtime) = &budget.income.overtime {
        for (rate_str, hours) in overtime {
            let rate: f64 = rate_str.parse().unwrap_or(1.0);
            let over = hours * hourly_pay * rate;
            overtime_pay += over;
        }
    }

    let gross_pay = budget.income.salary + budget.income.allowance + 
                    overtime_pay + budget.income.bonus + budget.income.misc + budget.income.pension;

    let it = tax(gross_pay, budget.tax.br_threshold / 12.0, 
                 budget.tax.br_rate, budget.tax.hr_threshold / 12.0, budget.tax.hr_rate);

    let ni = tax(gross_pay, budget.tax.p_threshold / 12.0, 
                 budget.tax.p_rate, budget.tax.u_threshold / 12.0, budget.tax.u_rate);

    let net_pay = gross_pay - (it + ni);

    let total_expenses: f64 = budget.expenses.iter().map(|e| *e.1).sum();

    let surplus: f64 = net_pay - total_expenses;

    println!("{:<23} {:>6}", "Income", "Amount");

    println!("{:<23} £ {:>9.2}", "Salary", budget.income.salary);
    println!("{:<23} £ {:>9.2}", "Allowance", budget.income.allowance);
    println!("{:<23} £ {:>9.2}", "Overtime", overtime_pay);
    println!("{:<23} £ {:>9.2}", "Bonus", budget.income.bonus);
    println!("{:<23} £ {:>9.2}", "Misc", budget.income.misc);
    println!("{:<23} £ {:>9.2}", "Pension", budget.income.pension);
    
    println!("\x1b[1m{:<23} £ {:>9.2}\x1b[0m", "Gross Pay", gross_pay);
    println!("\n{:<23} £ {:>9.2}", "Income Tax", it);
    println!("{:<23} £ {:>9.2}", "National Ins", ni);
    println!("\x1b[1m{:<23} £ {:>9.2}\x1b[0m", "Total Tax", it + ni);

    println!("\n\x1b[1m{:<23} £ {:>9.2}\x1b[0m", "Net Pay", net_pay);

    println!("\n{:<23} {:>6}", "Expense", "Amount");
    for (name, amount) in &budget.expenses {
        println!("{:<23} £ {:>9.2}", name, amount);
    }
    println!("\x1b[1m{:<23} £ {:>9.2}\x1b[0m", "Total Expenses", total_expenses);

    if surplus >=0.0 {
        println!("\n\x1b[32m{:<23} £ {:>9.2}\x1b[0m", "Surplus", surplus);
    } else {
        println!("\n\x1b[31m{:<23} £ {:>9.2}\x1b[0m", "Surplus", surplus);
    }
    Ok(())
}
