# Raculator - Command Line Calculator

## Project Overview

Raculator is a feature-rich command line calculator supporting standard arithmetic, bitwise operations, multiple number formats, and mathematical functions.

## Project Structure

```
raculator/
├── Cargo.toml              # Project configuration and dependencies
└── src/
    ├── main.rs            # Program entry point, CLI parsing and dispatch
    ├── lib.rs             # Test module
    ├── cli.rs             # Command line argument definitions (clap)
    ├── repl.rs            # Interactive mode (REPL)
    ├── calculator.rs      # Calculation core orchestration
    ├── evaluator.rs       # Postfix expression evaluation
    ├── functions.rs       # Mathematical function implementations
    ├── operator.rs        # Operator/function recognition and precedence
    ├── shunting_yard.rs   # Infix to postfix conversion algorithm
    ├── token.rs           # String tokenization
    └── linked_list.rs     # Linked list data structure
```

## Core Module Descriptions

### main.rs
- **Responsibility**: Program entry point, parse CLI arguments, dispatch to different modes
- **Key Functions**:
  - `main()`: Parse arguments and dispatch to calculation mode or interactive mode
  - `print_help()`: Display help information
  - `print_result()`: Format and output standard mode results
  - `print_bitwise_result()`: Format and output bitwise mode results

### cli.rs
- **Responsibility**: Command line argument definitions
- **Arguments**:
  - `expression`: Positional argument, expression to calculate directly
  - `-r, --row`: Expression argument (shorthand)
  - `-x, --hex`: Hexadecimal output
  - `-o, --oct`: Octal output
  - `-b, --bin`: Binary output
  - `-B, --bits`: Bitwise operation mode
  - `-F, --functions`: Display supported functions

### calculator.rs
- **Responsibility**: Calculation flow orchestration
- **Functions**:
  - `calculate()`: Standard mode calculation (f64)
  - `calculate_bitwise()`: Bitwise mode calculation (i64)
- **Flow**: Tokenize → Constant resolution → Linked list storage → Infix to postfix → Evaluation

### token.rs
- **Responsibility**: Lexical analysis, split input string into tokens
- **Supports**:
  - Scientific notation (1e3, 2.5e-3)
  - Number prefixes: 0b (binary), 0o (octal), 0x (hexadecimal)
  - Constants: pi, e
  - Bitwise operators (bitwise mode)
- **Key Functions**:
  - `tokenize()`: Main tokenization function
  - `resolve_constants()`: Parse constants and prefixed numbers

### shunting_yard.rs
- **Responsibility**: Infix to postfix expression conversion (Shunting Yard Algorithm)
- **Functions**:
  - `infix_to_postfix()`: Standard mode conversion
  - `infix_to_postfix_bitwise()`: Bitwise mode conversion

### evaluator.rs
- **Responsibility**: Postfix expression evaluation
- **Functions**:
  - `eval_postfix()`: Standard mode evaluation (f64)
  - `eval_postfix_bitwise()`: Bitwise mode evaluation (i64)

### functions.rs
- **Responsibility**: Mathematical function implementations
- **Function List**:
  - `eval_log_base()`: Custom base logarithm
  - `eval_lg()`: Base 10 logarithm
  - `eval_ln()`: Natural logarithm
  - `eval_sqrt()`: Square root
  - `eval_pow()`: Power function
  - `eval_sin/cos/tan()`: Trigonometric functions
  - `eval_asin/acos/atan()`: Inverse trigonometric functions
  - `eval_mod()`: Modulo function

### repl.rs
- **Responsibility**: Interactive calculator
- **Features**:
  - History input (arrow keys to browse)
  - Alt+G / @ to insert last result
  - hex/oct/bin commands for format conversion
- **Structures**:
  - `run_repl()`: Standard mode
  - `run_repl_bitwise()`: Bitwise mode

### operator.rs
- **Responsibility**: Operator and function recognition
- **Functions**:
  - `is_operator()`: Check if token is an operator
  - `is_bitwise_operator()`: Check if token is a bitwise operator
  - `is_function()`: Check if token is a function
  - `precedence()`: Operator precedence
  - `bitwise_precedence()`: Bitwise operator precedence

### linked_list.rs
- **Responsibility**: Linked list data structure (as per design requirement)
- **Structures**:
  - `Node`: Linked list node
  - `LinkedList`: Linked list structure

## Features

### Standard Operators
| Operator | Description | Precedence |
|----------|-------------|------------|
| `+` | Addition | 1 |
| `-` | Subtraction | 1 |
| `*` | Multiplication | 2 |
| `/` | Division | 2 |
| `%` | Modulo | 2 |
| `^` | Exponentiation | 3 |

### Bitwise Operators (-B mode)
| Operator | Description | Precedence |
|----------|-------------|------------|
| `\|` | Bitwise OR | 1 |
| `^` | Bitwise XOR | 2 |
| `&` | Bitwise AND | 3 |
| `<<` | Left shift | 4 |
| `>>` | Right shift | 4 |
| `~` | Bitwise NOT | 5 |

### Mathematical Functions
| Function | Description | Arguments |
|----------|-------------|-----------|
| `lg(x)` | Base 10 logarithm | 1 or 2 |
| `lg(x, base)` | Custom base logarithm | 2 |
| `log(x, base)` | Custom base logarithm | 2 |
| `ln(x)` | Natural logarithm | 1 |
| `sqrt(x)` | Square root | 1 |
| `pow(x, y)` | Power function | 2 |
| `sin(x)` | Sine | 1 |
| `cos(x)` | Cosine | 1 |
| `tan(x)` | Tangent | 1 |
| `asin(x)` | Inverse sine | 1 |
| `acos(x)` | Inverse cosine | 1 |
| `atan(x)` | Inverse tangent | 1 |
| `mod(a, b)` | Modulo function | 2 |

### Constants
- `pi` / `PI`: Pi ≈ 3.14159
- `e` / `E`: Euler's number ≈ 2.71828

### Number Formats
- Decimal: `123`, `-456`, `3.14`
- Binary: `0b1010`, `-0b1100`
- Octal: `0o755`, `-0o123`
- Hexadecimal: `0xFF`, `-0x1A`
- Scientific notation: `1e3`, `2.5e-3`, `1.23E+10`

## Usage Examples

### Command Line Mode
```bash
# Basic calculation
cargo run -- "1 + 2 * 3"           # Output: 7

# Using functions
cargo run -- "lg(100)"             # Output: 2
cargo run -- "sin(pi / 2)"         # Output: 1

# Bitwise operations
cargo run -- -B "12 & 10"          # Output: 8
cargo run -- -Bx "0xFF & 0x0F"     # Output: 0xF

# Base conversion
cargo run -- -x "255"              # Output: 0xFF
cargo run -- -o "255"              # Output: 0o377
cargo run -- -b "255"              # Output: 0b11111111

# View help
cargo run -- -F
cargo run -- --help
```

### Interactive Mode
```bash
cargo run
```

Interactive mode commands:
- `q` / `quit`: Exit
- `@`: Insert last result
- `Alt+G`: Insert last result
- `hex`: Show last result in hexadecimal
- `oct`: Show last result in octal
- `bin`: Show last result in binary

Bitwise interactive mode:
```bash
cargo run -- -B
```

## Data Flow

```
Input String
    ↓
tokenize() → Vec<String>
    ↓
resolve_constants() → Vec<String>
    ↓
LinkedList::push_back() → LinkedList
    ↓
to_vec() → Vec<String>
    ↓
infix_to_postfix() → Vec<String> (postfix expression)
    ↓
eval_postfix() → Result<T, String>
    ↓
Output Result
```

## Testing

Run all tests:
```bash
cargo test
```

Test coverage:
- Basic arithmetic (9 tests)
- Number formats (6 tests)
- Constants (3 tests)
- Function calculations (12 tests)
- Trigonometric functions (6 tests)
- Bitwise operations (8 tests)
- Error handling (7 tests)
- Lexical analysis (6 tests)
- Syntax analysis (3 tests)
- Data structures (2 tests)
- Output formats (3 tests)
- Integration tests (5 tests)

**Total: 61 test cases**

## Dependencies

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # CLI argument parsing
rustyline = "14.0"                                  # Interactive input
```

## Performance Optimizations

1. **Reduce cloning**: Use `std::mem::take()` instead of `clone()` + `clear()`
2. **Pattern matching**: Use `match` instead of nested `if-else`
3. **Early return**: Reduce unnecessary conditional checks
4. **Function extraction**: Improve code reusability and readability

## Code Standards

- All code formatted with `cargo fmt`
- All code passes `cargo clippy` checks
- English comments and documentation
- Clear function naming expressing intent
- Single responsibility for modules

## Extension Suggestions

1. **Add more functions**: Implement in `functions.rs`, register in `operator.rs`
2. **Support more bases**: Modify `parse_prefixed_number()` in `token.rs`
3. **Add variable support**: Extend `token.rs` and `evaluator.rs`
4. **Expression history**: Add history feature in `repl.rs`
5. **Custom precision**: Add floating-point precision control options

## FAQ

**Q: Why use linked list?**
A: Linked list is used to store expression tokens as per design requirement. While Vec is more efficient, linked list serves educational purposes.

**Q: Why use i64 for bitwise operations?**
A: i64 provides sufficient bit width and supports negative number bitwise operations.

**Q: How to add new functions?**
A: 
1. Implement the function in `functions.rs`
2. Add function name to `is_function()` in `operator.rs`
3. Add processing logic in `eval_postfix()` in `evaluator.rs`
4. Add description to help information in `main.rs`

## Author

Developed in 2026
