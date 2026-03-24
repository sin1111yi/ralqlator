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

//! REPL (Read-Eval-Print Loop) module
//!
//! Provides interactive calculator modes for both standard and bitwise operations.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Config, Context, Editor,
};

use crate::calculator::{calculate_bitwise, calculate_with_functions};
use crate::functions::UserFunctions;

/// Available REPL commands
const COMMANDS: &[&str] = &[
    "help", "functions", "operators", "formats", "constants", "mode", "hex", "oct", "bin",
    "create", "q", "quit",
];

/// Command completer for rustyline
struct CommandCompleter;

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut start = 0;

        // Find the start of the current word
        for (i, c) in line.chars().enumerate() {
            if c.is_whitespace() && i < pos {
                start = i + 1;
            } else if c.is_whitespace() && i >= pos {
                break;
            }
        }

        let word = &line[start..pos];
        let word_lower = word.to_lowercase();

        let mut candidates = Vec::new();
        for &cmd in COMMANDS {
            if cmd.starts_with(&word_lower.as_str()) {
                candidates.push(Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                });
            }
        }

        Ok((start, candidates))
    }
}

/// Custom helper implementing all required traits
struct Helper {
    completer: CommandCompleter,
}

impl Completer for Helper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for Helper {
    type Hint = String;
}

impl Highlighter for Helper {}

impl Validator for Helper {}

impl rustyline::Helper for Helper {}

/// Calculator mode
#[derive(Clone, Copy, PartialEq)]
enum CalcMode {
    Standard,
    Bitwise,
}

impl CalcMode {
    fn toggle(&mut self) {
        *self = match self {
            CalcMode::Standard => CalcMode::Bitwise,
            CalcMode::Bitwise => CalcMode::Standard,
        };
    }

    fn from_str(s: &str) -> Option<CalcMode> {
        match s {
            "standard" | "std" | "s" => Some(CalcMode::Standard),
            "bitwise" | "bit" | "b" => Some(CalcMode::Bitwise),
            _ => None,
        }
    }
}

/// Shared state for last result (standard mode - f64)
pub struct LastResult {
    value: Mutex<Option<f64>>,
}

impl LastResult {
    pub fn new() -> Self {
        LastResult {
            value: Mutex::new(None),
        }
    }

    pub fn set(&self, val: f64) {
        *self.value.lock().unwrap() = Some(val);
    }

    pub fn get(&self) -> Option<f64> {
        *self.value.lock().unwrap()
    }
}

/// Shared state for last result (bitwise mode - i64)
pub struct LastResultI64 {
    value: Mutex<Option<i64>>,
}

impl LastResultI64 {
    pub fn new() -> Self {
        LastResultI64 {
            value: Mutex::new(None),
        }
    }

    pub fn set(&self, val: i64) {
        *self.value.lock().unwrap() = Some(val);
    }

    pub fn get(&self) -> Option<i64> {
        *self.value.lock().unwrap()
    }
}

/// Print help for mathematical functions
pub fn print_functions_help() {
    println!("Mathematical Functions:");
    println!();
    println!("  lg(x)              Base-10 logarithm");
    println!("  lg(x, base)        Custom base logarithm");
    println!("  log(x, base)       Custom base logarithm (alias)");
    println!("  ln(x)              Natural logarithm (base e)");
    println!("  sqrt(x)            Square root");
    println!("  pow(x, y)          Power function (x^y)");
    println!("  sin(x)             Sine (x in radians)");
    println!("  cos(x)             Cosine (x in radians)");
    println!("  tan(x)             Tangent (x in radians)");
    println!("  asin(x)            Inverse sine (result in radians)");
    println!("  acos(x)            Inverse cosine (result in radians)");
    println!("  atan(x)            Inverse tangent (result in radians)");
    println!("  mod(a, b)          Modulo (a % b)");
    println!("  factorial(n)       Factorial (n!)");
    println!("  sum(a,b,...)       Sum of multiple arguments");
}

/// Print help for user-defined functions
pub fn print_user_functions_help(user_functions: &UserFunctions) {
    let funcs = user_functions.lock().unwrap();
    if funcs.is_empty() {
        println!("No user-defined functions.");
        return;
    }
    
    println!("User-defined Functions:");
    println!();
    for (name, (params, expr)) in funcs.iter() {
        let params_str = params.join(", ");
        println!("  {}({}) = {}", name, params_str, expr);
    }
    println!();
}

/// Print help for operators
pub fn print_operators_help() {
    println!("Operators:");
    println!();
    println!("  Standard Operators:");
    println!("    +    Addition           3 + 2 = 5");
    println!("    -    Subtraction        5 - 2 = 3");
    println!("    *    Multiplication     3 * 4 = 12");
    println!("    /    Division          10 / 2 = 5");
    println!("    %    Modulo            10 % 3 = 1");
    println!("    ^    Exponentiation     2 ^ 3 = 8");
    println!("    !    Factorial          5! = 120");
    println!();
    println!("  Bitwise Operators (use -B flag):");
    println!("    &    AND               12 & 10 = 8");
    println!("    |    OR                12 | 10 = 14");
    println!("    ^    XOR               12 ^ 10 = 6");
    println!("    ~    NOT               ~0 = -1");
    println!("    <<   Left shift        8 << 2 = 32");
    println!("    >>   Right shift       8 >> 2 = 2");
}

/// Print help for standard operators
fn print_standard_operators_help() {
    println!("Standard Operators:");
    println!();
    println!("    +    Addition           3 + 2 = 5");
    println!("    -    Subtraction        5 - 2 = 3");
    println!("    *    Multiplication     3 * 4 = 12");
    println!("    /    Division          10 / 2 = 5");
    println!("    %    Modulo            10 % 3 = 1");
    println!("    ^    Exponentiation     2 ^ 3 = 8");
}

/// Print help for bitwise operators
fn print_bitwise_operators_help() {
    println!("Bitwise Operators:");
    println!();
    println!("    &    AND               12 & 10 = 8");
    println!("    |    OR                12 | 10 = 14");
    println!("    ^    XOR               12 ^ 10 = 6");
    println!("    ~    NOT               ~0 = -1");
    println!("    <<   Left shift        8 << 2 = 32");
    println!("    >>   Right shift       8 >> 2 = 2");
}

/// Print help for number formats
pub fn print_formats_help() {
    println!("Number Formats:");
    println!();
    println!("  Input Formats:");
    println!("    Decimal     255, -456, 3.14");
    println!("    Binary      0b1010, 0b11111111");
    println!("    Octal       0o755, 0o377");
    println!("    Hexadecimal 0xFF, 0x1A");
    println!("    Scientific  1e3, 2.5e-3, 1.23E+10");
    println!();
    println!("  Output Format Options:");
    println!("    -x, --hex   Hexadecimal output  (e.g., 0xFF)");
    println!("    -o, --oct   Octal output        (e.g., 0o377)");
    println!("    -b, --bin   Binary output       (e.g., 0b11111111)");
}

/// Print help for constants
pub fn print_constants_help() {
    println!("Mathematical Constants:");
    println!();
    println!("  pi, PI    π ≈ 3.141592653589793  (ratio of circumference to diameter)");
    println!("  e, E      e ≈ 2.718281828459045  (Euler's number, natural log base)");
}

/// Print all help information
pub fn print_all_help() {
    print_operators_help();
    println!();
    print_functions_help();
    println!();
    print_constants_help();
    println!();
    print_formats_help();
}

/// Print help message for empty line Tab
fn print_tab_help() {
    println!("\n=== Available Commands ===");
    println!("  help [mode]       - Show help (mode: standard|bitwise)");
    println!("  functions [mode]  - Show function help");
    println!("  operators [mode]  - Show operator help for current/specified mode");
    println!("  formats           - Show number format help");
    println!("  constants         - Show mathematical constants");
    println!("  mode [mode]       - Switch mode (standard|bitwise) or toggle");
    println!("  hex/oct/bin       - Show last result in different formats");
    println!("  @                 - Insert last result");
    println!("  create function   - Define custom function: create function f(x,y) = x+y");
    println!("  create sequence   - Define sequence: create sequence a(n) = n*(n+1)/2");
    println!("  Call function     - Use: name(args), e.g., f(3,4) or a(10)");
    println!("  functions         - Show built-in and user-defined functions");
    println!("  q/quit            - Exit");
    println!();
    println!("  Tab               - Command completion");
    println!("  Enter (empty)     - Show this help");
    println!();
}

/// Run interactive REPL with mode switching support
fn run_repl_with_mode(initial_mode: CalcMode) {
    let mut mode = initial_mode;

    match mode {
        CalcMode::Standard => {
            println!("Type 'q' or 'quit' to exit. Press Tab for completion, Enter on empty line for help.\n");
        }
        CalcMode::Bitwise => {
            println!("Bitwise mode - Type 'q' or 'quit' to exit");
            println!("Operators: & (AND), | (OR), ^ (XOR), ~ (NOT), << (left shift), >> (right shift)");
            println!("Press Tab for completion, Enter on empty line for help.\n");
        }
    }

    let helper = Helper {
        completer: CommandCompleter,
    };
    let config = Config::builder().build();
    let mut rl: Editor<Helper, _> = Editor::with_config(config).expect("Failed to create readline editor");
    rl.set_helper(Some(helper));

    let last_result = Arc::new(LastResult::new());
    let last_result_i64 = Arc::new(LastResultI64::new());
    let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let input = rl.readline("> ");

        let Ok(input) = input else {
            break;
        };

        let input_trimmed = input.trim();
        
        // Handle empty input (show help on empty line)
        if input_trimmed.is_empty() {
            print_tab_help();
            continue;
        }
        
        rl.add_history_entry(input_trimmed).unwrap();

        let input_lower = input_trimmed.to_lowercase();

        // Handle mode command
        if input_lower.starts_with("mode") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 1 {
                // Toggle mode
                mode.toggle();
            } else if args.len() == 2 {
                // Set specific mode
                match args[1] {
                    "standard" | "std" | "s" => {
                        if mode != CalcMode::Standard {
                            mode = CalcMode::Standard;
                        }
                    }
                    "bitwise" | "bit" | "b" => {
                        if mode != CalcMode::Bitwise {
                            mode = CalcMode::Bitwise;
                        }
                    }
                    _ => {
                        eprintln!("Usage: mode [standard|bitwise]\n");
                        continue;
                    }
                }
            } else {
                eprintln!("Usage: mode [standard|bitwise]\n");
                continue;
            }

            match mode {
                CalcMode::Standard => {
                    println!("Switched to Standard mode (floating-point arithmetic)");
                    println!("Operators: +, -, *, /, %, ^ (exponentiation)\n");
                }
                CalcMode::Bitwise => {
                    println!("Switched to Bitwise mode (integer operations)");
                    println!("Operators: & (AND), | (OR), ^ (XOR), ~ (NOT), <<, >>\n");
                }
            }
            continue;
        }

        // Handle create function/sequence commands
        if input_lower.starts_with("create ") {
            let parts: Vec<&str> = input_trimmed.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let cmd = parts[1].to_lowercase();
                let rest = parts[2];
                
                if cmd == "function" || cmd == "func" || cmd == "fn" {
                    // Parse: create function name(args) = expression
                    if let Some(eq_pos) = rest.find('=') {
                        let name_part = rest[..eq_pos].trim();
                        let expr = rest[eq_pos + 1..].trim();
                        
                        // Parse name and arguments
                        if let Some(paren_start) = name_part.find('(') {
                            if let Some(paren_end) = name_part.find(')') {
                                let name = name_part[..paren_start].trim().to_string();
                                let args_str = name_part[paren_start + 1..paren_end].trim();
                                let args: Vec<String> = args_str.split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                
                                let mut funcs = user_functions.lock().unwrap();
                                funcs.insert(name.clone(), (args, expr.to_string()));
                                println!("Function '{}' defined\n", name);
                                continue;
                            }
                        }
                    }
                    eprintln!("Usage: create function name(args) = expression\n");
                    continue;
                } else if cmd == "sequence" || cmd == "seq" {
                    // Parse: create sequence name(n) = expression
                    if let Some(eq_pos) = rest.find('=') {
                        let name_part = rest[..eq_pos].trim();
                        let expr = rest[eq_pos + 1..].trim();
                        
                        // Parse name and argument (should be single variable like n)
                        if let Some(paren_start) = name_part.find('(') {
                            if let Some(paren_end) = name_part.find(')') {
                                let name = name_part[..paren_start].trim().to_string();
                                let arg = name_part[paren_start + 1..paren_end].trim().to_string();
                                
                                // Store as a function with single argument
                                let mut funcs = user_functions.lock().unwrap();
                                funcs.insert(name.clone(), (vec![arg], expr.to_string()));
                                println!("Sequence '{}' defined: {}(n) = {}\n", name, name, expr);
                                continue;
                            }
                        }
                    }
                    eprintln!("Usage: create sequence name(n) = expression\n");
                    continue;
                }
            }
            eprintln!("Usage: create function name(args) = expression\n");
            eprintln!("       create sequence name(n) = expression\n");
            continue;
        }

        // Handle help commands
        if input_lower == "help" || input_lower.starts_with("help ") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 1 {
                print_all_help();
                print_user_functions_help(&user_functions);
            } else if args.len() == 2 {
                match CalcMode::from_str(args[1]) {
                    Some(CalcMode::Standard) => {
                        print_standard_operators_help();
                        println!();
                        print_functions_help();
                        println!();
                        print_constants_help();
                        println!();
                        print_formats_help();
                    }
                    Some(CalcMode::Bitwise) => {
                        print_bitwise_operators_help();
                        println!();
                        print_formats_help();
                        println!("\nNote: Mathematical functions are not available in bitwise mode.");
                    }
                    None => {
                        eprintln!("Usage: help [standard|bitwise]\n");
                        continue;
                    }
                }
            } else {
                eprintln!("Usage: help [standard|bitwise]\n");
                continue;
            }
            println!();
            continue;
        } else if input_lower == "functions" || input_lower.starts_with("functions ") {
            print_functions_help();
            print_user_functions_help(&user_functions);
            println!();
            continue;
        } else if input_lower == "operators" || input_lower.starts_with("operators ") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 1 {
                // Show operators for current mode
                match mode {
                    CalcMode::Standard => print_standard_operators_help(),
                    CalcMode::Bitwise => print_bitwise_operators_help(),
                }
            } else if args.len() == 2 {
                match CalcMode::from_str(args[1]) {
                    Some(CalcMode::Standard) => print_standard_operators_help(),
                    Some(CalcMode::Bitwise) => print_bitwise_operators_help(),
                    None => {
                        eprintln!("Usage: operators [standard|bitwise]\n");
                        continue;
                    }
                }
            } else {
                eprintln!("Usage: operators [standard|bitwise]\n");
                continue;
            }
            println!();
            continue;
        } else if input_lower == "formats" || input_lower.starts_with("formats ") {
            print_formats_help();
            println!();
            continue;
        } else if input_lower == "constants" || input_lower.starts_with("constants ") {
            print_constants_help();
            println!();
            continue;
        }

        // Handle format conversion commands
        if matches!(input_lower.as_str(), "hex" | "oct" | "bin") {
            if let Some(val) = last_result_i64.get() {
                print_formatted(val, input_lower.as_str());
            } else if let Some(val) = last_result.get() {
                if val.fract() == 0.0 && val.abs() < 1e15 {
                    print_formatted(val as i64, input_lower.as_str());
                } else {
                    eprintln!("Error: format conversion only supports integer results");
                }
            } else {
                eprintln!("No previous result available");
            }
            continue;
        }

        // Replace placeholders
        let processed = match replace_placeholders_any(&input, mode, last_result.get(), last_result_i64.get()) {
            Some(s) => s,
            None => continue,
        };

        let processed = processed.trim();

        if processed.is_empty() {
            continue;
        }

        if processed.eq_ignore_ascii_case("q") || processed.eq_ignore_ascii_case("quit") {
            break;
        }

        // Calculate based on current mode
        match mode {
            CalcMode::Standard => {
                match calculate_with_functions(processed, &user_functions) {
                    Ok(result) => {
                        last_result.set(result);
                        println!("{}", result);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            CalcMode::Bitwise => {
                match calculate_bitwise(processed) {
                    Ok(result) => {
                        last_result_i64.set(result);
                        println!("{}", result);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }
}

/// Run interactive REPL mode (standard mode) - legacy function
pub fn run_repl() {
    run_repl_with_mode(CalcMode::Standard);
}

/// Run interactive REPL mode (bitwise mode) - legacy function
pub fn run_repl_bitwise() {
    run_repl_with_mode(CalcMode::Bitwise);
}

/// Print value in specified format
fn print_formatted(val: i64, format: &str) {
    match format {
        "hex" => {
            if val < 0 {
                println!("-0x{:X}", val.unsigned_abs());
            } else {
                println!("0x{:X}", val as u64);
            }
        }
        "oct" => {
            if val < 0 {
                println!("-0o{:o}", val.unsigned_abs());
            } else {
                println!("0o{:o}", val as u64);
            }
        }
        "bin" => {
            if val < 0 {
                println!("-0b{:b}", val.unsigned_abs());
            } else {
                println!("0b{:b}", val as u64);
            }
        }
        _ => {}
    }
}

/// Replace placeholders supporting both modes
fn replace_placeholders_any(
    input: &str,
    mode: CalcMode,
    last_result: Option<f64>,
    last_result_i64: Option<i64>,
) -> Option<String> {
    if input.contains("__LAST_RESULT__") || input.contains('@') {
        let val = match mode {
            CalcMode::Standard => last_result.map(|v| v.to_string()),
            CalcMode::Bitwise => last_result_i64.map(|v| v.to_string()),
        };
        if let Some(val_str) = val {
            Some(
                input
                    .replace("__LAST_RESULT__", &val_str)
                    .replace('@', &val_str),
            )
        } else {
            eprintln!("No previous result available");
            None
        }
    } else {
        Some(input.to_string())
    }
}
