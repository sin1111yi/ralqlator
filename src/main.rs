// Ralqlator - A command line calculator
//
// Copyright (C) 2026 Ralqlator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod calculator;
mod cli;
mod error;
mod evaluator;
mod functions;
mod operator;
mod parser;
mod rational;
mod repl;
mod shunting_yard;
mod storage;
mod token;
mod value;

use std::process;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use clap::Parser;
use cli::Cli;

// Re-export help functions from repl module
use repl::{
    print_all_help, print_constants_help, print_formats_help, print_functions_help,
    print_operators_help,
};
use storage::{load_user_data, save_user_data, delete_user_definition};
use functions::UserFunctions;
use repl::UserConstants;

/// Print version information
fn print_version() {
    let git_tag = env!("GIT_TAG");
    let git_commit = env!("GIT_COMMIT");
    let git_date = env!("GIT_DATE");
    let rust_version = env!("RUST_VERSION");
    let build_time = env!("BUILD_TIME");
    
    println!("Ralqlator {}", git_tag);
    println!("Git Commit: {} ({})", git_commit, git_date);
    println!("Build Time: {}", build_time);
    println!("Rust Version: {}", rust_version);
    println!();
    println!("Copyright (C) 2026 Ralqlator Contributors");
    println!("License: GPLv3");
}

/// Format integer as string with specified base prefix
/// Negative numbers are represented in two's complement form (no sign prefix)
fn format_int_base(val: i64, hex: bool, oct: bool, bin: bool) -> String {
    // Always use unsigned representation (two's complement for negative numbers)
    let unsigned_val = val as u64;
    if hex {
        format!("0x{:X}", unsigned_val)
    } else if oct {
        format!("0o{:o}", unsigned_val)
    } else if bin {
        format!("0b{:b}", unsigned_val)
    } else {
        val.to_string()
    }
}

/// Check if expression contains scientific notation
fn contains_scientific_notation(expr: &str) -> bool {
    // Check for 'e' or 'E' followed by optional sign and digits
    let chars: Vec<char> = expr.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == 'e' || chars[i] == 'E' {
            // Check if it's part of a number (preceded by digit or dot)
            if i > 0 && (chars[i - 1].is_ascii_digit() || chars[i - 1] == '.') {
                return true;
            }
        }
    }
    false
}

/// Check if a small decimal number has at least 8 leading zeros after decimal point
/// Returns true for numbers like 0.000000001 (1e-9) or smaller
/// Note: 0.00000001 has 7 zeros, 0.000000001 has 8 zeros
fn has_many_leading_zeros(value: f64) -> bool {
    if value == 0.0 {
        return false;
    }
    let abs_val = value.abs();
    // Check if absolute value is <= 1e-7 (has 7+ leading zeros, displays as 0.0000000X or smaller)
    // This means numbers like 0.00000001 (1e-8) and smaller will use scientific notation
    abs_val <= 1e-7 && abs_val > 0.0
}

/// Format and print result based on output format flags
fn print_result(
    result: f64,
    hex: bool,
    oct: bool,
    bin: bool,
    original_expr: &str,
) -> Result<(), String> {
    // Check for special comparison results first
    if let Some(comparison_str) = evaluator::format_comparison_result(result) {
        println!("{}", comparison_str);
        return Ok(());
    }

    // Check if input contained scientific notation
    let input_had_scientific = contains_scientific_notation(original_expr);

    // Check if result has many leading zeros (8+ zeros after decimal point)
    let has_leading_zeros = has_many_leading_zeros(result);

    if hex || oct || bin {
        if result.fract() == 0.0 && result.abs() < 1e15 {
            println!("{}", format_int_base(result as i64, hex, oct, bin));
            Ok(())
        } else {
            Err("Base output only supports integer results".to_string())
        }
    } else if result.fract() == 0.0 && result.abs() < 1e15 {
        let int_result = result as i64;
        println!("{}", int_result);
        // If input had scientific notation, also show scientific format
        if input_had_scientific {
            println!("  (scientific: {:.6e})", result);
        }
        Ok(())
    } else {
        // For very small numbers with 8+ leading zeros, use scientific notation
        if has_leading_zeros {
            println!("{:.6e}", result);
        } else {
            println!("{}", result);
        }
        // If input had scientific notation, also show scientific format
        if input_had_scientific && !has_leading_zeros {
            println!("  (scientific: {:.6e})", result);
        }
        Ok(())
    }
}

/// Format and print bitwise result based on output format flags
fn print_bitwise_result(result: i64, hex: bool, oct: bool, bin: bool) {
    println!("{}", format_int_base(result, hex, oct, bin));
}

fn main() {
    let cli = Cli::parse();

    // Handle version flag (-v, -V, --version)
    if std::env::args().any(|arg| arg == "-v" || arg == "-V" || arg == "--version") {
        print_version();
        return;
    }

    // Handle help display commands
    match cli.command {
        Some(cli::Commands::Functions) => {
            print_functions_help();
            return;
        }
        Some(cli::Commands::Operators) => {
            print_operators_help();
            return;
        }
        Some(cli::Commands::Formats) => {
            print_formats_help();
            return;
        }
        Some(cli::Commands::Constants) => {
            print_constants_help();
            return;
        }
        Some(cli::Commands::Info) => {
            print_all_help();
            return;
        }
        None => {}
    }

    // Handle flag-based help
    if cli.show_functions {
        print_functions_help();
        return;
    }

    if cli.show_operators {
        print_operators_help();
        return;
    }

    if cli.show_formats {
        print_formats_help();
        return;
    }

    if cli.show_constants {
        print_constants_help();
        return;
    }

    // Handle user definition commands (--create, --destroy, --list)
    if cli.list {
        // List all user definitions
        let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));
        let user_constants: UserConstants = Arc::new(Mutex::new(HashMap::new()));
        
        match load_user_data(&user_functions, &user_constants) {
            Ok(count) => {
                if count > 0 {
                    println!("User definitions ({} items):", count);
                    println!();
                    
                    let funcs = user_functions.lock().unwrap();
                    if !funcs.is_empty() {
                        println!("Functions:");
                        for (name, (params, expr)) in funcs.iter() {
                            println!("  {}({}) = {}", name, params.join(", "), expr);
                        }
                        println!();
                    }
                    drop(funcs);
                    
                    let consts = user_constants.lock().unwrap();
                    if !consts.is_empty() {
                        println!("Constants:");
                        for (name, value) in consts.iter() {
                            println!("  {} = {}", name, value);
                        }
                        println!();
                    }
                } else {
                    println!("No user definitions found.");
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    if let Some(create_def) = &cli.create {
        // Create user definition from command line
        let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));
        let user_constants: UserConstants = Arc::new(Mutex::new(HashMap::new()));
        
        // Load existing definitions
        let _ = load_user_data(&user_functions, &user_constants);
        
        // Parse: func name(args) = expr | seq name(n) = expr | const NAME value
        let create_lower = create_def.to_lowercase();
        let mut created = false;
        
        if create_lower.starts_with("func ") || create_lower.starts_with("function ") || 
           create_lower.starts_with("fn ") || create_lower.starts_with("f ") {
            // Parse function definition
            if let Some(eq_pos) = create_def.find('=') {
                let header = create_def[..eq_pos].trim();
                let expr = create_def[eq_pos + 1..].trim();
                
                // Extract name and params: name(args)
                if let Some(paren_start) = header.find('(') {
                    if let Some(paren_end) = header.find(')') {
                        let name = header[..paren_start].trim();
                        // Remove the type prefix (func/function/fn/f)
                        let name = name.split_whitespace().last().unwrap_or(name);
                        let args_str = header[paren_start + 1..paren_end].trim();
                        let params: Vec<String> = args_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        
                        match calculator::create_user_function(name, params, expr.to_string(), &user_functions) {
                            Ok(()) => {
                                match save_user_data(&user_functions, &user_constants) {
                                    Ok(()) => println!("Function '{}' created and saved", name),
                                    Err(e) => eprintln!("Error: Function created but failed to save: {}", e),
                                }
                                created = true;
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
        } else if create_lower.starts_with("seq ") || create_lower.starts_with("sequence ") ||
                  create_lower.starts_with("s ") {
            // Parse sequence definition
            if let Some(eq_pos) = create_def.find('=') {
                let header = create_def[..eq_pos].trim();
                let expr = create_def[eq_pos + 1..].trim();
                
                if let Some(paren_start) = header.find('(') {
                    if let Some(paren_end) = header.find(')') {
                        let name = header[..paren_start].trim();
                        let name = name.split_whitespace().last().unwrap_or(name);
                        let param = header[paren_start + 1..paren_end].trim().to_string();
                        
                        match calculator::create_user_sequence(name, param, expr.to_string(), &user_functions) {
                            Ok(()) => {
                                match save_user_data(&user_functions, &user_constants) {
                                    Ok(()) => println!("Sequence '{}' created and saved", name),
                                    Err(e) => eprintln!("Error: Sequence created but failed to save: {}", e),
                                }
                                created = true;
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
        } else if create_lower.starts_with("const ") || create_lower.starts_with("c ") ||
                  create_lower.starts_with("constant ") {
            // Parse constant definition: const NAME value
            let parts: Vec<&str> = create_def.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[1];
                if let Ok(value) = parts[2].parse::<f64>() {
                    match calculator::create_user_constant(name, value, &user_constants) {
                        Ok(()) => {
                            match save_user_data(&user_functions, &user_constants) {
                                Ok(()) => println!("Constant '{}' = {} created and saved", name, value),
                                Err(e) => eprintln!("Error: Constant created but failed to save: {}", e),
                            }
                            created = true;
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    eprintln!("Error: Invalid number: '{}'", parts[2]);
                }
            }
        }
        
        if !created && !create_def.is_empty() {
            eprintln!("Usage: -c \"func name(args) = expression\"");
            eprintln!("       -c \"seq name(n) = expression\"");
            eprintln!("       -c \"const NAME value\"");
            process::exit(1);
        }
        return;
    }

    if let Some(name) = &cli.destroy {
        // Delete user definition
        let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));
        let user_constants: UserConstants = Arc::new(Mutex::new(HashMap::new()));
        
        let _ = load_user_data(&user_functions, &user_constants);
        
        match delete_user_definition(name, &user_functions, &user_constants) {
            Ok(true) => println!("Definition '{}' deleted", name),
            Ok(false) => {
                eprintln!("Definition '{}' not found", name);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    // Handle expression calculation
    if let Some(expr) = cli.expression.or(cli.row) {
        if cli.bits {
            // Bitwise mode
            match calculator::calculate_bitwise(&expr) {
                Ok(result) => print_bitwise_result(result, cli.hex, cli.oct, cli.bin),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        } else {
            // Standard mode - load user definitions first
            let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));
            let user_constants: UserConstants = Arc::new(Mutex::new(HashMap::new()));
            let _ = load_user_data(&user_functions, &user_constants);
            
            match calculator::calculate_with_functions(&expr, &user_functions, &user_constants) {
                Ok(result) => {
                    if let Err(e) = print_result(result, cli.hex, cli.oct, cli.bin, &expr) {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        return;
    }

    // Interactive mode
    if cli.bits {
        repl::run_repl_bitwise();
    } else {
        repl::run_repl();
    }
}
