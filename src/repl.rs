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

use crate::calculator::{
    calculate_bitwise, calculate_with_functions, create_user_constant, create_user_function,
    create_user_sequence,
};
use crate::evaluator::format_comparison_result;
use crate::functions::UserFunctions;
use crate::storage::{self, load_user_data, save_user_data};

/// User-defined constants type
pub type UserConstants = Arc<Mutex<HashMap<String, f64>>>;

/// Available REPL commands
const COMMANDS: &[&str] = &[
    "help",
    "functions",
    "operators",
    "formats",
    "constants",
    "mode",
    "hex",
    "oct",
    "bin",
    "create",
    "destroy",
    "q",
    "quit",
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
            if cmd.starts_with(word_lower.as_str()) {
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

impl Default for LastResult {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for LastResultI64 {
    fn default() -> Self {
        Self::new()
    }
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
    println!("Mathematical Functions");
    println!("======================");
    println!();
    println!("  Logarithms:");
    println!("    lg(x)                Base-10 logarithm");
    println!("    lg(x, base)          Custom base logarithm");
    println!("    log(x, base)         Custom base logarithm (alias)");
    println!("    ln(x)                Natural logarithm (base e)");
    println!("    log2(x)              Base-2 logarithm");
    println!();
    println!("  Roots & Powers:");
    println!("    sqrt(x)              Square root");
    println!("    cbrt(x)              Cube root");
    println!("    pow(x, y)            Power function (x^y)");
    println!();
    println!("  Trigonometric:");
    println!("    sin/cos/tan(x)       Sine/Cosine/Tangent (radians)");
    println!("    sec/csc/cot(x)       Secant/Cosecant/Cotangent");
    println!("    asin/acos/atan(x)    Inverse trig functions");
    println!("    atan2(y, x)          Two-argument arctangent");
    println!();
    println!("  Hyperbolic:");
    println!("    sinh/cosh/tanh(x)    Hyperbolic sine/cosine/tangent");
    println!("    asinh/acosh/atanh(x) Inverse hyperbolic functions");
    println!();
    println!("  Special Functions:");
    println!("    factorial(n)         Factorial (n!, max 170!)");
    println!("    gamma(n)             Gamma function");
    println!("    erf/erfc(x)          Error functions");
    println!("    beta(x, y)           Beta function");
    println!();
    println!("  BigInt Functions (arbitrary precision):");
    println!("    bfactorial(n)        Big factorial (max 10000!)");
    println!("    bpow(base, exp)      Big power (max exp 1000)");
    println!("    comb(n, k)           Combinations C(n,k)");
    println!("    perm(n, k)           Permutations P(n,k)");
    println!("    gcd(a, b)            Greatest common divisor");
    println!("    lcm(a, b)            Least common multiple");
    println!("    isprime(n)           Prime check (1=true, 0=false)");
    println!("    nextprime(n)         Next prime after n");
    println!();
    println!("  Utility Functions:");
    println!("    mod(a, b)            Modulo (a % b)");
    println!("    sum(a,b,...)         Sum of multiple arguments");
    println!("    prod(a,b,...)        Product of arguments");
    println!("    abs/floor/ceil(x)    Absolute/floor/ceiling");
    println!("    round(x)             Round to nearest integer");
    println!();
    println!("  Sequence Functions:");
    println!("    suma(s,b,e)          Sum of sequence s from b to e");
    println!();
    println!("  Rational Functions:");
    println!("    num(x)               Get numerator of rational");
    println!("    den(x)               Get denominator of rational");
    println!("    frac(x)              Get fractional part");
    println!("    rational(n,d)        Create rational from n/d");
    println!("    float(x)             Convert to floating point");
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
    println!("Operators");
    println!("=========");
    println!();
    println!("  Arithmetic Operators:");
    println!("    +    Addition           3 + 2 = 5");
    println!("    -    Subtraction        5 - 2 = 3");
    println!("    *    Multiplication     3 * 4 = 12");
    println!("    /    Division           10 / 2 = 5");
    println!("    %    Modulo             10 % 3 = 1");
    println!("    ^    Exponentiation     2 ^ 3 = 8");
    println!("    !    Factorial          5! = 120");
    println!();
    println!("  Comparison Operators:");
    println!("    <    Less than          3 < 5 → true");
    println!("    >    Greater than       5 > 3 → true");
    println!("    =    Equality           5 = 5 → true");
    println!("    ==   Logical equality   5 == 5 → true");
    println!();
    println!("  Bitwise Operators (-B mode):");
    println!("    &    AND                12 & 10 = 8");
    println!("    |    OR                 12 | 10 = 14");
    println!("    ^    XOR                12 ^ 10 = 6");
    println!("    ~    NOT                ~0 = -1");
    println!("    <<   Left shift         8 << 2 = 32");
    println!("    >>   Right shift        8 >> 2 = 2");
    println!();
    println!("  Operator Precedence (highest to lowest):");
    println!("    1. ! (factorial)");
    println!("    2. ^ (exponentiation, right-associative)");
    println!("    3. * / % (multiplication, division, modulo)");
    println!("    4. + - (addition, subtraction)");
    println!("    5. < > = == (comparison, lowest)");
}

/// Print help for standard operators
fn print_standard_operators_help() {
    println!("Standard Operators:");
    println!();
    println!("  Arithmetic:");
    println!("    +    Addition           3 + 2 = 5");
    println!("    -    Subtraction        5 - 2 = 3");
    println!("    *    Multiplication     3 * 4 = 12");
    println!("    /    Division           10 / 2 = 5");
    println!("    %    Modulo             10 % 3 = 1");
    println!("    ^    Exponentiation     2 ^ 3 = 8");
    println!("    !    Factorial          5! = 120");
    println!();
    println!("  Comparison:");
    println!("    <    Less than          3 < 5 → true");
    println!("    >    Greater than       5 > 3 → true");
    println!("    =    Equality           5 = 5 → true");
    println!("    ==   Logical equality   5 == 5 → true");
}

/// Print help for bitwise operators
fn print_bitwise_operators_help() {
    println!("Bitwise Operators:");
    println!();
    println!("    &    AND                12 & 10 = 8");
    println!("    |    OR                 12 | 10 = 14");
    println!("    ^    XOR                12 ^ 10 = 6");
    println!("    ~    NOT                ~0 = -1");
    println!("    <<   Left shift         8 << 2 = 32");
    println!("    >>   Right shift        8 >> 2 = 2");
}

/// Print help for number formats
pub fn print_formats_help() {
    println!("Number Formats:");
    println!();
    println!("  Input Formats:");
    println!("    Decimal      255, -456, 3.14");
    println!("    Binary       0b1010, 0b11111111");
    println!("    Octal        0o755, 0o377");
    println!("    Hexadecimal  0xFF, 0x1A");
    println!("    Scientific   1e3, 2.5e-3, 1.23E+10");
    println!();
    println!("  Output Format Options:");
    println!("    -x, --hex    Hexadecimal output (e.g., 0xFF)");
    println!("    -o, --oct    Octal output (e.g., 0o377)");
    println!("    -b, --bin    Binary output (e.g., 0b11111111)");
}

/// Print help for create command
pub fn print_create_help() {
    println!("Create Command - Define custom functions, sequences, and constants");
    println!();
    println!("  Usage:");
    println!("    create <func|seq|const> name = value/expression");
    println!();
    println!("  Subcommands:");
    println!("    func, function, fn, f    Define a function");
    println!("    seq, sequence, s         Define a sequence (single-variable function)");
    println!("    const, c, constant       Define a constant");
    println!();
    println!("  Examples:");
    println!("    create func f(x) = x + 1           Simple function");
    println!("    create func add(a, b) = a + b      Two arguments");
    println!("    create func square(x) = x * x      Square function");
    println!("    create seq triangle(n) = n(n+1)/2  Triangle numbers");
    println!("    create const MY_CONST 3.14159      Define a constant");
    println!("    create const G 9.81                Gravity constant");
    println!();
    println!("  Notes:");
    println!("    - Function/sequence/constant names starting with C_ are reserved");
    println!("    - Use suma(seq, begin, end) to sum sequence terms");
    println!("    - Constants are simple name-value pairs (no parameters)");
    println!();
}

/// Print help for constants
pub fn print_constants_help() {
    println!("Mathematical Constants:");
    println!();
    println!("  C_PI    π ≈ 3.141592653589793  (ratio of circumference to diameter)");
    println!("  C_E     e ≈ 2.718281828459045  (Euler's number, natural log base)");
    println!();
    println!("  Note:");
    println!("    Constants use C_xx format (C_PI, C_E).");
    println!("    Names starting with C_ are reserved and cannot be used");
    println!("    for user-defined functions or sequences.");
}

/// Print detailed help for constants
pub fn print_constants_help_detailed() {
    println!("Constants - Mathematical Constants");
    println!();
    println!("  Built-in Constants:");
    println!("    C_PI    π ≈ 3.141592653589793");
    println!("    C_E     e ≈ 2.718281828459045");
    println!();
    println!("  User-defined Constants:");
    println!("    create const NAME VALUE    Define a constant");
    println!();
    println!("  Examples:");
    println!("    create const G 9.81");
    println!("    create const C 299792458");
    println!();
    println!("  Usage:");
    println!("    C_PI * 2         Use constant in expression");
    println!("    C_E ^ 2          Use constant in function");
    println!();
}

/// Print detailed help for formats
pub fn print_formats_help_detailed() {
    println!("Formats - Number Format Support");
    println!();
    println!("  Input Formats:");
    println!("    Decimal      255, -456, 3.14");
    println!("    Binary       0b1010, 0b11111111");
    println!("    Octal        0o755, 0o377");
    println!("    Hexadecimal  0xFF, 0x1A");
    println!("    Scientific   1e3, 2.5e-3, 1.23E+10");
    println!();
    println!("  Output Format Options:");
    println!("    -x, --hex    Hexadecimal output");
    println!("    -o, --oct    Octal output");
    println!("    -b, --bin    Binary output");
    println!();
    println!("  Interactive Commands:");
    println!("    hex          Show last result in hexadecimal");
    println!("    oct          Show last result in octal");
    println!("    bin          Show last result in binary");
    println!();
}

/// Print detailed help for functions
pub fn print_functions_help_detailed() {
    println!("Functions - Mathematical Functions");
    println!();
    println!("  Logarithms:");
    println!("    lg(x)                Base-10 logarithm");
    println!("    lg(x, base)          Custom base logarithm");
    println!("    log(x, base)         Custom base logarithm");
    println!("    ln(x)                Natural logarithm");
    println!("    log2(x)              Base-2 logarithm");
    println!();
    println!("  Roots & Powers:");
    println!("    sqrt(x)              Square root");
    println!("    cbrt(x)              Cube root");
    println!("    pow(x, y)            Power function");
    println!();
    println!("  Trigonometric:");
    println!("    sin/cos/tan(x)       Trigonometric functions");
    println!("    sec/csc/cot(x)       Reciprocal trig functions");
    println!("    asin/acos/atan(x)    Inverse trigonometric");
    println!("    atan2(y, x)          Two-argument arctangent");
    println!();
    println!("  Hyperbolic:");
    println!("    sinh/cosh/tanh(x)    Hyperbolic functions");
    println!("    asinh/acosh/atanh(x) Inverse hyperbolic");
    println!();
    println!("  Special Functions:");
    println!("    factorial(n)         Factorial (n!, max 170!)");
    println!("    gamma(n)             Gamma function Γ(n)");
    println!("    erf/erfc(x)          Error functions");
    println!("    beta(x, y)           Beta function B(x,y)");
    println!();
    println!("  BigInt Functions (arbitrary precision integers):");
    println!("    bfactorial(n)        Big factorial (max 10000!)");
    println!("      Example: bfactorial(50) = 30414093... (65 digits)");
    println!("    bpow(base, exp)      Big power (max exp 1000)");
    println!("      Example: bpow(2, 100) = 1267650600... (31 digits)");
    println!("    comb(n, k)           Combinations C(n,k)");
    println!("      Example: comb(52, 5) = 2598960 (poker hands)");
    println!("    perm(n, k)           Permutations P(n,k)");
    println!("      Example: perm(10, 3) = 720");
    println!("    gcd(a, b)            Greatest common divisor");
    println!("      Example: gcd(48, 18) = 6");
    println!("    lcm(a, b)            Least common multiple");
    println!("      Example: lcm(12, 18) = 36");
    println!("    isprime(n)           Prime check (1=true, 0=false)");
    println!("      Example: isprime(17) = 1");
    println!("    nextprime(n)         Next prime after n");
    println!("      Example: nextprime(100) = 101");
    println!();
    println!("  Utility Functions:");
    println!("    mod(a, b)            Modulo");
    println!("    sum(a,b,...)         Sum of arguments");
    println!("    prod(a,b,...)        Product of arguments");
    println!("    abs/floor/ceil(x)    Absolute/floor/ceiling");
    println!("    round(x)             Round to nearest integer");
    println!();
    println!("  Sequence Functions:");
    println!("    suma(s,b,e)          Sum of sequence s from b to e");
    println!();
    println!("  User-defined Functions:");
    println!("    create func f(x) = expression");
    println!();
}

/// Print detailed help for operators
pub fn print_operators_help_detailed() {
    println!("Operators - Supported Operators");
    println!();
    println!("  Arithmetic Operators:");
    println!("    +    Addition           3 + 2 = 5");
    println!("    -    Subtraction        5 - 2 = 3");
    println!("    *    Multiplication     3 * 4 = 12");
    println!("    /    Division           10 / 2 = 5");
    println!("    %    Modulo             10 % 3 = 1");
    println!("    ^    Exponentiation     2 ^ 3 = 8");
    println!("    !    Factorial          5! = 120");
    println!();
    println!("  Comparison Operators:");
    println!("    <    Less than          3 < 5 → true");
    println!("    >    Greater than       5 > 3 → true");
    println!("    =    Equality           5 = 5 → true");
    println!("    ==   Logical equality   5 == 5 → true");
    println!();
    println!("  Bitwise Operators (-B mode):");
    println!("    &    AND                12 & 10 = 8");
    println!("    |    OR                 12 | 10 = 14");
    println!("    ^    XOR                12 ^ 10 = 6");
    println!("    ~    NOT                ~0 = -1");
    println!("    <<   Left shift         8 << 2 = 32");
    println!("    >>   Right shift        8 >> 2 = 2");
    println!();
    println!("  Notes:");
    println!("    - Comparison operators have the lowest precedence");
    println!("    - < and > return true/false (relational comparison)");
    println!("    - = returns true/false (equality check)");
    println!("    - == returns true/false (logical equality)");
    println!();
}

/// Print help for mode command
pub fn print_mode_help() {
    println!("Mode - Switch Between Calculation Modes");
    println!();
    println!("  Usage:");
    println!("    mode               Toggle between standard and bitwise");
    println!("    mode standard      Switch to standard mode");
    println!("    mode bitwise       Switch to bitwise mode");
    println!();
    println!("  Modes:");
    println!("    standard    Floating-point arithmetic (+, -, *, /, %, ^)");
    println!("    bitwise     Integer operations (&, |, ^, ~, <<, >>)");
    println!();
}

/// Print help for help command (main help display)
pub fn print_help_command() {
    println!("=== Available Commands ===");
    println!();
    println!("  Help Commands:");
    println!("    help                 Show this help");
    println!("    help functions       Show detailed function help");
    println!("    help operators       Show detailed operator help");
    println!("    help formats         Show detailed format help");
    println!("    help constants       Show detailed constant help");
    println!("    help mode            Show mode help");
    println!("    help create          Show create command help");
    println!("    help standard        Show standard mode help");
    println!("    help bitwise         Show bitwise mode help");
    println!();
    println!("  Quick Commands:");
    println!("    functions            Show function list");
    println!("    operators [mode]     Show operators (mode: standard|bitwise)");
    println!("    formats              Show number formats");
    println!("    constants            Show mathematical constants");
    println!("    mode [mode]          Switch mode or toggle");
    println!("    hex/oct/bin          Convert last result format");
    println!("    @                    Insert last result");
    println!("    create <f|s|c> ...   Define function, sequence, or constant");
    println!("    destroy <name>       Delete a definition by name");
    println!("    q/quit               Exit");
    println!();
    println!("  Tips:");
    println!("    Tab                  Command completion");
    println!("    Enter (empty line)   Show this help");
    println!();
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
    print_help_command();
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
            println!(
                "Operators: & (AND), | (OR), ^ (XOR), ~ (NOT), << (left shift), >> (right shift)"
            );
            println!("Press Tab for completion, Enter on empty line for help.\n");
        }
    }

    let helper = Helper {
        completer: CommandCompleter,
    };
    let config = Config::builder().build();
    let mut rl: Editor<Helper, _> =
        Editor::with_config(config).expect("Failed to create readline editor");
    rl.set_helper(Some(helper));

    let last_result = Arc::new(LastResult::new());
    let last_result_i64 = Arc::new(LastResultI64::new());
    let user_functions: UserFunctions = Arc::new(Mutex::new(HashMap::new()));
    let user_constants: UserConstants = Arc::new(Mutex::new(HashMap::new()));

    // Auto-load user definitions from storage file
    match load_user_data(&user_functions, &user_constants) {
        Ok(count) => {
            if count > 0 {
                println!("Loaded {} definitions from {}\n", count, storage::get_storage_path_string());
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to load saved definitions: {}\n", e);
        }
    }

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
            if args.len() >= 2 && (args[1] == "--help" || args[1] == "-h") {
                print_mode_help();
                continue;
            } else if args.len() == 1 {
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

        // Handle create command
        if input_lower.starts_with("create") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 1 {
                // No subcommand: show usage
                eprintln!("Usage: create <func|seq|const> name = value/expression");
                eprintln!("  Examples:");
                eprintln!("    create func f(x) = x + 1");
                eprintln!("    create seq a(n) = n * 2");
                eprintln!("    create const MY_CONST 3.14\n");
                continue;
            } else if args.len() >= 2 && (args[1] == "--help" || args[1] == "-h") {
                // Show detailed help
                print_create_help();
                continue;
            } else if args.len() >= 2 {
                let subcmd = args[1];

                // Get the rest of the input after "create <subcmd> "
                let prefix_len = format!("create {}", subcmd).len();
                let rest = if input_trimmed.len() > prefix_len {
                    input_trimmed[prefix_len..].trim()
                } else {
                    ""
                };

                if subcmd == "func" || subcmd == "function" || subcmd == "fn" || subcmd == "f" {
                    // Check if in bitwise mode - functions are not supported
                    if matches!(mode, CalcMode::Bitwise) {
                        eprintln!("Error: User-defined functions are not supported in bitwise mode.");
                        eprintln!("Bitwise mode only supports: & (AND), | (OR), ^ (XOR), ~ (NOT), <<, >>");
                        eprintln!("Switch to standard mode first: mode standard\n");
                        continue;
                    }
                    
                    // Parse: create func name(args) = expression
                    if let Some(eq_pos) = rest.find('=') {
                        let name_part = rest[..eq_pos].trim();
                        let expr = rest[eq_pos + 1..].trim();

                        // Parse name and arguments
                        if let Some(paren_start) = name_part.find('(') {
                            if let Some(paren_end) = name_part.find(')') {
                                let name = name_part[..paren_start].trim().to_string();
                                let args_str = name_part[paren_start + 1..paren_end].trim();
                                let params: Vec<String> = args_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                match create_user_function(
                                    &name,
                                    params,
                                    expr.to_string(),
                                    &user_functions,
                                ) {
                                    Ok(()) => {
                                        println!("Function '{}' defined\n", name);
                                        // Auto-save after creation
                                        if let Err(e) = save_user_data(&user_functions, &user_constants) {
                                            eprintln!("Warning: Failed to auto-save: {}\n", e);
                                        }
                                    }
                                    Err(e) => eprintln!("Error: {}\n", e),
                                }
                                continue;
                            }
                        }
                    }
                    eprintln!("Usage: create func name(args) = expression\n");
                    continue;
                } else if subcmd == "seq" || subcmd == "sequence" || subcmd == "s" {
                    // Check if in bitwise mode - sequences are not supported
                    if matches!(mode, CalcMode::Bitwise) {
                        eprintln!("Error: User-defined sequences are not supported in bitwise mode.");
                        eprintln!("Bitwise mode only supports: & (AND), | (OR), ^ (XOR), ~ (NOT), <<, >>");
                        eprintln!("Switch to standard mode first: mode standard\n");
                        continue;
                    }
                    
                    // Parse: create seq name(n) = expression
                    if let Some(eq_pos) = rest.find('=') {
                        let name_part = rest[..eq_pos].trim();
                        let expr = rest[eq_pos + 1..].trim();

                        // Parse name and argument (should be single variable like n)
                        if let Some(paren_start) = name_part.find('(') {
                            if let Some(paren_end) = name_part.find(')') {
                                let name = name_part[..paren_start].trim().to_string();
                                let param =
                                    name_part[paren_start + 1..paren_end].trim().to_string();

                                match create_user_sequence(
                                    &name,
                                    param,
                                    expr.to_string(),
                                    &user_functions,
                                ) {
                                    Ok(()) => {
                                        println!("Sequence '{}' defined: {}(n) = {}\n", name, name, expr);
                                        // Auto-save after creation
                                        if let Err(e) = save_user_data(&user_functions, &user_constants) {
                                            eprintln!("Warning: Failed to auto-save: {}\n", e);
                                        }
                                    }
                                    Err(e) => eprintln!("Error: {}\n", e),
                                }
                                continue;
                            }
                        }
                    }
                    eprintln!("Usage: create seq name(n) = expression\n");
                    continue;
                } else if subcmd == "c" || subcmd == "const" || subcmd == "constant" {
                    // Parse: create c NAME value
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        match parts[1].parse::<f64>() {
                            Ok(value) => {
                                match create_user_constant(&name, value, &user_constants) {
                                    Ok(()) => {
                                        println!("Constant '{}' = {}\n", name, value);
                                        // Auto-save after creation
                                        if let Err(e) = save_user_data(&user_functions, &user_constants) {
                                            eprintln!("Warning: Failed to auto-save: {}\n", e);
                                        }
                                    }
                                    Err(e) => eprintln!("Error: {}\n", e),
                                }
                            }
                            Err(_) => eprintln!("Error: Invalid number: '{}'\n", parts[1]),
                        }
                        continue;
                    }
                    eprintln!("Usage: create c NAME value\n");
                    continue;
                } else {
                    eprintln!("Unknown create subcommand: '{}'\n", subcmd);
                    eprintln!("Usage: create <func|seq|const> ...\n");
                    continue;
                }
            }
            continue;
        }

        // Handle destroy command
        if input_lower == "destroy" || input_lower.starts_with("destroy ") {
            // Use original input to preserve case sensitivity for the name
            let orig_args: Vec<&str> = input_trimmed.split_whitespace().collect();
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            
            if args.len() == 1 || (args.len() >= 2 && (args[1] == "--help" || args[1] == "-h")) {
                println!("Destroy Command - Delete user-defined functions, sequences, or constants");
                println!();
                println!("  Usage:");
                println!("    destroy <name>         Delete a definition by name");
                println!("    destroy --help         Show this help");
                println!();
                println!("  Description:");
                println!("    Deletes a user-defined function, sequence, or constant");
                println!("    by name and auto-saves the changes.");
                println!();
                println!("  Examples:");
                println!("    destroy f              Delete function 'f'");
                println!("    destroy MY_CONST       Delete constant 'MY_CONST'");
                println!("    destroy dBuV           Delete function 'dBuV' (case-sensitive)");
                println!();
            } else if args.len() == 2 {
                // Use original case name from orig_args
                let name = orig_args[1];
                match storage::delete_user_definition(name, &user_functions, &user_constants) {
                    Ok(true) => println!("Deleted '{}' and saved\n", name),
                    Ok(false) => eprintln!("Definition '{}' not found\n", name),
                    Err(e) => eprintln!("Error: {}\n", e),
                }
            } else {
                eprintln!("Usage: destroy <name>\n");
            }
            continue;
        }

        // Handle help command
        if input_lower == "help" || input_lower.starts_with("help ") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 1 {
                print_help_command();
            } else if args.len() == 2 {
                // Check for specific help topics
                match args[1] {
                    "create" => print_create_help(),
                    "functions" | "func" => {
                        print_functions_help_detailed();
                        print_user_functions_help(&user_functions);
                    }
                    "operators" | "op" => print_operators_help_detailed(),
                    "formats" | "format" => print_formats_help_detailed(),
                    "constants" | "const" => print_constants_help_detailed(),
                    "mode" => print_mode_help(),
                    "standard" | "std" | "s" => {
                        print_standard_operators_help();
                        println!();
                        print_functions_help();
                        println!();
                        print_constants_help();
                        println!();
                        print_formats_help();
                    }
                    "bitwise" | "bit" | "b" => {
                        print_bitwise_operators_help();
                        println!();
                        print_formats_help();
                        println!(
                            "\nNote: Mathematical functions are not available in bitwise mode."
                        );
                    }
                    _ => {
                        eprintln!("Unknown help topic: '{}'\n", args[1]);
                        eprintln!("Available topics: standard, bitwise, create, functions, operators, formats, constants, mode\n");
                        continue;
                    }
                }
            } else {
                eprintln!("Usage: help [topic]\n");
                eprintln!("Available topics: standard, bitwise, create, functions, operators, formats, constants, mode\n");
                continue;
            }
            println!();
            continue;
        } else if input_lower == "functions" || input_lower.starts_with("functions") {
            print_functions_help();
            print_user_functions_help(&user_functions);
            println!();
            continue;
        } else if input_lower == "operators" || input_lower.starts_with("operators") {
            let args: Vec<&str> = input_lower.split_whitespace().collect();
            if args.len() == 2 {
                match CalcMode::from_str(args[1]) {
                    Some(CalcMode::Standard) => print_standard_operators_help(),
                    Some(CalcMode::Bitwise) => print_bitwise_operators_help(),
                    None => {
                        eprintln!("Usage: operators [standard|bitwise]\n");
                        continue;
                    }
                }
            } else {
                // Show operators for current mode
                match mode {
                    CalcMode::Standard => print_standard_operators_help(),
                    CalcMode::Bitwise => print_bitwise_operators_help(),
                }
            }
            println!();
            continue;
        } else if input_lower == "formats" || input_lower.starts_with("formats") {
            print_formats_help();
            println!();
            continue;
        } else if input_lower == "constants" || input_lower.starts_with("constants") {
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
        let processed = match replace_placeholders_any(
            &input,
            mode,
            last_result.get(),
            last_result_i64.get(),
        ) {
            Some(s) => s,
            None => continue,
        };

        let processed = processed.trim();

        if processed.is_empty() {
            continue;
        }

        if processed.eq_ignore_ascii_case("q") || processed.eq_ignore_ascii_case("quit") {
            // Auto-save user definitions before exiting
            match save_user_data(&user_functions, &user_constants) {
                Ok(()) => println!("User definitions saved to {}", storage::get_storage_path_string()),
                Err(e) => eprintln!("Warning: Failed to save: {}", e),
            }
            break;
        }

        // Calculate based on current mode
        match mode {
            CalcMode::Standard => {
                match calculate_with_functions(processed, &user_functions, &user_constants) {
                    Ok(result) => {
                        last_result.set(result);
                        // Check for special comparison results
                        if let Some(comparison_str) = format_comparison_result(result) {
                            println!("{}", comparison_str);
                        } else {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            CalcMode::Bitwise => match calculate_bitwise(processed) {
                Ok(result) => {
                    last_result_i64.set(result);
                    println!("{}", result);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
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
/// Negative numbers are represented in two's complement form (no sign prefix)
fn print_formatted(val: i64, format: &str) {
    // Always use unsigned representation (two's complement for negative numbers)
    let unsigned_val = val as u64;
    match format {
        "hex" => println!("0x{:X}", unsigned_val),
        "oct" => println!("0o{:o}", unsigned_val),
        "bin" => println!("0b{:b}", unsigned_val),
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
