//! Raculator - A command line calculator
//!
//! Supports standard arithmetic and bitwise operations with multiple number formats.

mod calculator;
mod cli;
mod evaluator;
mod functions;
mod linked_list;
mod operator;
mod repl;
mod shunting_yard;
mod token;

use std::process;

use clap::Parser;
use cli::Cli;

/// Print help information for functions and features
fn print_help() {
    println!("Supported operators:");
    println!("  +  -  *  /  %  ^");
    println!();
    println!("Supported functions:");
    println!("  lg(x)         - Base 10 logarithm");
    println!("  log(x, base)  - Custom base logarithm");
    println!("  ln(x)         - Natural logarithm (base e)");
    println!("  sqrt(x)       - Square root");
    println!("  pow(x, y)     - Power function (x^y)");
    println!("  sin(x)        - Sine (radians)");
    println!("  cos(x)        - Cosine (radians)");
    println!("  tan(x)        - Tangent (radians)");
    println!("  asin(x)       - Inverse sine (radians)");
    println!("  acos(x)       - Inverse cosine (radians)");
    println!("  atan(x)       - Inverse tangent (radians)");
    println!("  mod(a, b)     - Modulo (a % b)");
    println!();
    println!("Constants:");
    println!("  pi, PI        - π ≈ 3.14159");
    println!("  e, E          - e ≈ 2.71828 (Euler's number)");
    println!();
    println!("Number formats:");
    println!("  Decimal:      123, -456, 3.14");
    println!("  Binary:       0b1010, -0b1100");
    println!("  Octal:        0o755, -0o123");
    println!("  Hexadecimal:  0xFF, -0x1A");
    println!();
    println!("Bitwise mode (-B, --bits):");
    println!("  &   - Bitwise AND");
    println!("  |   - Bitwise OR");
    println!("  ^   - Bitwise XOR");
    println!("  ~   - Bitwise NOT");
    println!("  <<  - Left shift");
    println!("  >>  - Right shift");
    println!();
    println!("Output formats:");
    println!("  -x, --hex     Hexadecimal output");
    println!("  -o, --oct     Octal output");
    println!("  -b, --bin     Binary output");
}

/// Format and print result based on output format flags
fn print_result(result: f64, hex: bool, oct: bool, bin: bool) -> Result<(), String> {
    if hex || oct || bin {
        if result.fract() == 0.0 && result.abs() < 1e15 {
            let int_val = result as i64;
            if hex {
                if int_val < 0 {
                    println!("= -0x{:X}", int_val.unsigned_abs());
                } else {
                    println!("= 0x{:X}", int_val as u64);
                }
            } else if oct {
                if int_val < 0 {
                    println!("= -0o{:o}", int_val.unsigned_abs());
                } else {
                    println!("= 0o{:o}", int_val as u64);
                }
            } else if bin {
                if int_val < 0 {
                    println!("= -0b{:b}", int_val.unsigned_abs());
                } else {
                    println!("= 0b{:b}", int_val as u64);
                }
            }
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
    if hex {
        if result < 0 {
            println!("= -0x{:X}", result.unsigned_abs());
        } else {
            println!("= 0x{:X}", result as u64);
        }
    } else if oct {
        if result < 0 {
            println!("= -0o{:o}", result.unsigned_abs());
        } else {
            println!("= 0o{:o}", result as u64);
        }
    } else if bin {
        if result < 0 {
            println!("= -0b{:b}", result.unsigned_abs());
        } else {
            println!("= 0b{:b}", result as u64);
        }
    } else {
        println!("{}", result);
    }
}

fn main() {
    let cli = Cli::parse();

    // Handle help display
    if cli.show_functions || matches!(cli.command, Some(cli::Commands::Functions)) {
        print_help();
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
