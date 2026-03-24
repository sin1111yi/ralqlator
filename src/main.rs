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
mod evaluator;
mod functions;
mod operator;
mod repl;
mod shunting_yard;
mod token;

use std::process;

use clap::Parser;
use cli::Cli;

// Re-export help functions from repl module
use repl::{
    print_all_help, print_constants_help, print_formats_help, print_functions_help,
    print_operators_help,
};

/// Format integer as string with specified base prefix
fn format_int_base(val: i64, hex: bool, oct: bool, bin: bool) -> String {
    if hex {
        if val < 0 {
            format!("-0x{:X}", val.unsigned_abs())
        } else {
            format!("0x{:X}", val as u64)
        }
    } else if oct {
        if val < 0 {
            format!("-0o{:o}", val.unsigned_abs())
        } else {
            format!("0o{:o}", val as u64)
        }
    } else if bin {
        if val < 0 {
            format!("-0b{:b}", val.unsigned_abs())
        } else {
            format!("0b{:b}", val as u64)
        }
    } else {
        val.to_string()
    }
}

/// Format and print result based on output format flags
fn print_result(result: f64, hex: bool, oct: bool, bin: bool) -> Result<(), String> {
    if hex || oct || bin {
        if result.fract() == 0.0 && result.abs() < 1e15 {
            println!("{}", format_int_base(result as i64, hex, oct, bin));
            Ok(())
        } else {
            Err("Base output only supports integer results".to_string())
        }
    } else if result.fract() == 0.0 && result.abs() < 1e15 {
        println!("{}", result as i64);
        Ok(())
    } else {
        println!("{}", result);
        Ok(())
    }
}

/// Format and print bitwise result based on output format flags
fn print_bitwise_result(result: i64, hex: bool, oct: bool, bin: bool) {
    println!("{}", format_int_base(result, hex, oct, bin));
}

fn main() {
    let cli = Cli::parse();

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
            // Standard mode
            match calculator::calculate(&expr) {
                Ok(result) => {
                    if let Err(e) = print_result(result, cli.hex, cli.oct, cli.bin) {
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
