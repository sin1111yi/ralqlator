# Git Commit Messages for Raculator

## Initial Setup
```
feat: initialize Rust project with cargo

- Create new Rust project structure
- Set up Cargo.toml with basic configuration
- Add initial main.rs with Hello World
```

## Core Calculator Implementation
```
feat: implement basic calculator with stack and linked list

- Add LinkedList data structure for expression storage
- Implement tokenizer for parsing input
- Add Shunting Yard algorithm for infix to postfix conversion
- Implement postfix expression evaluator
- Support basic operators: +, -, *, /, ^
- Add interactive REPL mode
```

```
feat: refactor codebase into modular structure

- Split main.rs into multiple modules:
  - calculator.rs: calculation orchestration
  - evaluator.rs: expression evaluation
  - functions.rs: mathematical functions
  - operator.rs: operator definitions
  - shunting_yard.rs: infix to postfix conversion
  - token.rs: lexical analysis
  - linked_list.rs: linked list implementation
  - cli.rs: CLI argument definitions
  - repl.rs: interactive REPL
```

```
feat: update all comments to English

- Convert all Chinese comments to English
- Maintain technical accuracy
- Keep code style consistent
```

## CLI and Input/Output
```
feat: add CLI argument support with clap

- Add expression as positional argument
- Add -r/--row shorthand for expressions
- Add -F/--functions to show supported functions
- Add -h/--help for help menu
- Support invalid argument detection
```

```
feat: integrate rustyline for interactive input

- Add arrow key history navigation
- Add left/right cursor movement
- Replace standard stdin with rustyline editor
- Support Ctrl+R history search
```

```
feat: add last result insertion feature

- Add Alt+G shortcut to insert last result
- Add @ symbol as shorthand for last result
- Implement LastResult state management
- Support result history in REPL
```

## Mathematical Functions
```
feat: add logarithm functions

- Add lg(x) for base-10 logarithm
- Add lg(x, base) for custom base logarithm
- Add log(x, base) for custom base logarithm
- Add ln(x) for natural logarithm
- Add input validation and error handling
```

```
feat: add power and square root functions

- Add pow(x, y) function for exponentiation
- Add sqrt(x) function for square root
- Support both function and operator (^) for power
- Add domain validation
```

```
feat: add trigonometric functions

- Add sin(x), cos(x), tan(x)
- Add asin(x), acos(x), atan(x)
- All functions use radians
- Add domain validation for inverse functions
```

## Constants and Number Formats
```
feat: add mathematical constants

- Add pi/PI constant (≈ 3.14159)
- Add e/E constant (≈ 2.71828)
- Support case-insensitive constant names
- Distinguish constant 'e' from scientific notation
```

```
feat: add scientific notation support

- Support format: 1e3, 2.5e-3, 1.23E+10
- Handle sign in exponent (e+, e-)
- Integrate with tokenizer
```

```
feat: add multiple number format input

- Add binary input with 0b prefix
- Add octal input with 0o prefix
- Add hexadecimal input with 0x prefix
- Support negative prefixed numbers
- Add parser for prefixed number conversion
```

```
feat: add base conversion output

- Add -x/--hex for hexadecimal output
- Add -o/--oct for octal output
- Add -b/--bin for binary output
- Support negative number formatting
- Only support integer results
```

## Additional Operations
```
feat: add modulo operation

- Add % operator for modulo
- Add mod(a, b) function
- Add division by zero protection
- Same precedence as multiplication
```

```
feat: add bitwise operation mode

- Add -B/--bits flag for bitwise mode
- Support operators: &, |, ^, ~, <<, >>
- Implement bitwise tokenizer
- Implement bitwise shunting yard
- Implement integer-only evaluator
- Add shift range validation (0-63)
```

```
feat: add format conversion commands in REPL

- Add hex command to show last result in hex
- Add oct command to show last result in octal
- Add bin command to show last result in binary
- Support in both standard and bitwise modes
- Update help text
```

## Code Quality
```
refactor: improve code readability and performance

- Extract helper functions in token.rs
- Use match expressions instead of if-else chains
- Add std::mem::take() to reduce cloning
- Simplify main.rs with extracted functions
- Add module documentation comments
- Reduce code from 1247 to 1189 lines
```

```
test: add comprehensive test suite

- Add 61 test cases covering:
  - Basic arithmetic (9 tests)
  - Number formats (6 tests)
  - Constants (3 tests)
  - Functions (12 tests)
  - Trigonometric (6 tests)
  - Bitwise operations (8 tests)
  - Error handling (7 tests)
  - Tokenizer (6 tests)
  - Parser (3 tests)
  - Data structures (2 tests)
  - Output formats (3 tests)
  - Integration (5 tests)
- Create lib.rs for test module
- All tests passing
```

## Documentation
```
docs: add agent documentation files

- Add agent.md (English version, 298 lines)
- Add agent_zh.md (Chinese version, 298 lines)
- Document project structure
- Document all modules and functions
- Include usage examples
- Add FAQ section
- Add extension suggestions
```
