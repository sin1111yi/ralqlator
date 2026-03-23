# Raculator Git Commit History

## Recommended Commit Sequence

For future reference or if you want to reorganize the git history, here's the recommended commit sequence:

```bash
# 1. Initial project setup
git add Cargo.toml
git commit -m "feat: initialize Rust project with cargo

- Create new Rust project structure
- Set up Cargo.toml with basic configuration"

# 2. Core calculator implementation
git add src/main.rs src/linked_list.rs src/token.rs src/shunting_yard.rs src/evaluator.rs src/operator.rs
git commit -m "feat: implement basic calculator with stack and linked list

- Add LinkedList data structure for expression storage
- Implement tokenizer for parsing input
- Add Shunting Yard algorithm for infix to postfix conversion
- Implement postfix expression evaluator
- Support basic operators: +, -, *, /, ^
- Add interactive REPL mode"

# 3. Modular refactoring
git add src/calculator.rs src/functions.rs src/cli.rs src/repl.rs
git commit -m "feat: refactor codebase into modular structure

- Split functionality into dedicated modules:
  - calculator.rs: calculation orchestration
  - functions.rs: mathematical function implementations
  - cli.rs: CLI argument definitions with clap
  - repl.rs: interactive REPL mode
- Improve code organization and maintainability"

# 4. English comments
git commit -am "docs: update all comments to English

- Convert all Chinese comments to English
- Maintain technical accuracy
- Keep code style consistent"

# 5. CLI support
git commit -am "feat: add CLI argument support with clap

- Add expression as positional argument
- Add -r/--row shorthand for expressions
- Add -F/--functions to show supported functions
- Add -h/--help for help menu
- Support invalid argument detection"

# 6. Readline integration
git commit -am "feat: integrate rustyline for interactive input

- Add arrow key history navigation
- Add left/right cursor movement
- Replace standard stdin with rustyline editor
- Support Ctrl+R history search"

# 7. Last result feature
git commit -am "feat: add last result insertion feature

- Add Alt+G shortcut to insert last result
- Add @ symbol as shorthand for last result
- Implement LastResult state management
- Support result history in REPL"

# 8. Logarithm functions
git commit -am "feat: add logarithm functions

- Add lg(x) for base-10 logarithm
- Add lg(x, base) for custom base logarithm
- Add log(x, base) for custom base logarithm
- Add ln(x) for natural logarithm
- Add input validation and error handling"

# 9. Power and sqrt functions
git commit -am "feat: add power and square root functions

- Add pow(x, y) function for exponentiation
- Add sqrt(x) function for square root
- Support both function and operator (^) for power
- Add domain validation"

# 10. Trigonometric functions
git commit -am "feat: add trigonometric functions

- Add sin(x), cos(x), tan(x)
- Add asin(x), acos(x), atan(x)
- All functions use radians
- Add domain validation for inverse functions"

# 11. Mathematical constants
git commit -am "feat: add mathematical constants

- Add pi/PI constant (≈ 3.14159)
- Add e/E constant (≈ 2.71828)
- Support case-insensitive constant names
- Distinguish constant 'e' from scientific notation"

# 12. Scientific notation
git commit -am "feat: add scientific notation support

- Support format: 1e3, 2.5e-3, 1.23E+10
- Handle sign in exponent (e+, e-)
- Integrate with tokenizer"

# 13. Multiple number formats
git commit -am "feat: add multiple number format input

- Add binary input with 0b prefix
- Add octal input with 0o prefix
- Add hexadecimal input with 0x prefix
- Support negative prefixed numbers
- Add parser for prefixed number conversion"

# 14. Base conversion output
git commit -am "feat: add base conversion output

- Add -x/--hex for hexadecimal output
- Add -o/--oct for octal output
- Add -b/--bin for binary output
- Support negative number formatting
- Only support integer results"

# 15. Modulo operation
git commit -am "feat: add modulo operation

- Add % operator for modulo
- Add mod(a, b) function
- Add division by zero protection
- Same precedence as multiplication"

# 16. Bitwise mode
git commit -am "feat: add bitwise operation mode

- Add -B/--bits flag for bitwise mode
- Support operators: &, |, ^, ~, <<, >>
- Implement bitwise tokenizer
- Implement bitwise shunting yard
- Implement integer-only evaluator
- Add shift range validation (0-63)"

# 17. Format conversion commands
git commit -am "feat: add format conversion commands in REPL

- Add hex command to show last result in hex
- Add oct command to show last result in octal
- Add bin command to show last result in binary
- Support in both standard and bitwise modes
- Update help text"

# 18. Code refactoring
git commit -am "refactor: improve code readability and performance

- Extract helper functions in token.rs
- Use match expressions instead of if-else chains
- Add std::mem::take() to reduce cloning
- Simplify main.rs with extracted functions
- Add module documentation comments
- Reduce code from 1247 to 1189 lines"

# 19. Test suite
git add src/lib.rs
git commit -m "test: add comprehensive test suite

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
- All tests passing"

# 20. Documentation
git add agent.md agent_zh.md
git commit -m "docs: add agent documentation files

- Add agent.md (English version, 298 lines)
- Add agent_zh.md (Chinese version, 298 lines)
- Document project structure
- Document all modules and functions
- Include usage examples
- Add FAQ section
- Add extension suggestions"
```

## Current Status

The project has been committed as a single initial commit. To reorganize into the above sequence, you would need to:

1. Reset to empty: `git reset --soft $(git hash-object -t tree /dev/null)`
2. Follow the commit sequence above

Or keep the current single commit and use `commits.md` as a reference for the development history.

## Quick Reference

| Commit | Feature | Files Changed |
|--------|---------|---------------|
| 1 | Project setup | Cargo.toml |
| 2 | Core calculator | main.rs, linked_list.rs, token.rs, shunting_yard.rs, evaluator.rs, operator.rs |
| 3 | Modular structure | calculator.rs, functions.rs, cli.rs, repl.rs |
| 4 | English docs | All .rs files |
| 5 | CLI support | cli.rs, main.rs |
| 6 | Readline | repl.rs, Cargo.toml |
| 7 | Last result | repl.rs |
| 8 | Logarithms | functions.rs, operator.rs, evaluator.rs |
| 9 | Power/sqrt | functions.rs |
| 10 | Trig functions | functions.rs |
| 11 | Constants | token.rs |
| 12 | Scientific notation | token.rs |
| 13 | Number formats | token.rs |
| 14 | Base output | main.rs, cli.rs |
| 15 | Modulo | operator.rs, evaluator.rs |
| 16 | Bitwise mode | All core modules |
| 17 | Format commands | repl.rs |
| 18 | Refactoring | All modules |
| 19 | Tests | lib.rs |
| 20 | Documentation | agent.md, agent_zh.md |
