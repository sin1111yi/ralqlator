# Ralqlator Test Documentation

> **Other Languages**: [中文](README_zh.md)

## Test File Structure (9 files, 397 tests)

| # | File | Tests | Description |
|------|--------|--------|------|
| 1 | `01_core_tests.rs` | 66 | Core functionality tests (arithmetic, bitwise, comparison, constants, edge cases) |
| 2 | `02_functions_tests.rs` | 35 | Mathematical function tests (logarithms, trigonometry, BigInt, etc.) |
| 3 | `03_cli_formats_tests.rs` | 33 | CLI arguments and number format tests |
| 4 | `04_repl_tests.rs` | 55 | REPL interactive mode tests |
| 5 | `05_rational_tests.rs` | 44 | Rational number and parser tests |
| 6 | `06_error_internal_tests.rs` | 46 | Error handling and internal module tests |
| 7 | `07_user_defined_tests.rs` | 8 | User-defined constant tests |
| 8 | `08_extended_tests.rs` | 76 | Extended comprehensive tests |
| 9 | `e2e_integration_tests.rs` | 34 | End-to-end integration tests |
| **Total** | **9 files** | **397** | Full coverage |

---

## Detailed Classification

### 1. Core Functionality Tests (`01_core_tests.rs` - 66 tests)

#### Arithmetic Operations
- Addition: basic, large numbers, negative, multiple additions
- Subtraction: basic, negative results, negative number subtraction
- Multiplication: basic, multiply by zero, negative multiplication
- Division: basic, decimal results, divide by one, division by zero errors
- Modulo: basic, modulo by zero errors
- Exponentiation: basic, zero exponent, first power
- Factorial: basic, 0!, 1!, large number factorial

#### Bitwise Operations
- AND, OR, XOR, NOT
- Left shift, right shift, shift by zero
- Combined operations, hexadecimal input

#### Comparison Operations
- Less than, greater than, equal, double equal
- True/false value tests, expression comparison

#### Constants
- C_PI, C_E
- Constant expressions, constants with functions

#### Edge Cases
- Zero operations, negative numbers, decimals, large numbers
- Chain operations, mixed operations, identity operations

### 2. Function Tests (`02_functions_tests.rs` - 35 tests)

#### Logarithm Functions
- lg (base 10), custom base
- ln (natural logarithm), log2

#### Root Functions
- sqrt (square root), cbrt (cube root)

#### Power Functions
- pow

#### Trigonometric Functions
- sin, cos, tan
- asin, acos, atan

#### Hyperbolic Functions
- sinh, cosh, tanh

#### Utility Functions
- abs, floor, ceil, round, mod
- sum, prod (multi-parameter)

#### BigInt Functions
- bfactorial, bpow
- comb, perm
- gcd, lcm
- isprime, nextprime

#### Error Tests
- Negative square root, negative logarithm
- Arcsin out of range

### 3. CLI and Format Tests (`03_cli_formats_tests.rs` - 33 tests)

#### Command Line Arguments
- Positional arguments, -r shorthand

#### Output Formats
- -x hexadecimal, -o octal, -b binary
- Format output calculations

#### Bitwise Mode
- -B flag, combined flags

#### Input Formats
- 0b binary, 0o octal, 0x hexadecimal
- Negative prefix numbers, mixed formats

#### Scientific Notation
- 1e3, 1e-3, scientific notation arithmetic

#### Help System
- --help, -F, -O, -N, -C
- Subcommands: functions, operators, formats, constants

#### Error Handling
- Non-integer base output errors, invalid expressions

### 4. REPL Tests (`04_repl_tests.rs` - 55 tests)

#### Basic Interaction
- Arithmetic operations, bitwise mode
- Function calls, constant usage

#### User Definitions
- Function definition and usage
- Sequence definition and suma summation
- Constant definition

#### History Results
- @ reference, chain operations
- hex/oct/bin conversion

#### Comparison Operations
- <, >, =, ==

#### Number Formats
- Hexadecimal, binary, octal input

#### Help Commands
- help, help functions, help operators
- help constants

#### Error Handling
- Division by zero, invalid expressions, undefined functions
- Mismatched parentheses

#### Edge Cases
- Empty lines, whitespace, negative numbers, nested parentheses

#### BigInt Functions
- bfactorial, bpow, gcd, lcm
- isprime, nextprime

### 5. Rational Tests (`05_rational_tests.rs` - 33 tests)

#### Parser Tests
- Number parsing (decimal, binary, octal, hexadecimal, scientific notation)
- Identifiers, function calls, AST node construction
- Expression parsing (simple, parentheses)

#### Rational CLI
- Fraction input, rational operations
- Rational function CLI

#### Rational REPL
- Fraction input interaction, rational operation interaction
- User-defined functions/constants with rationals

#### Parser Integration
- AST evaluation, parentheses, unary operators
- Exponentiation, comparison operations
- Fraction input, nested functions
- Error handling (division by zero, undefined functions)
- Right-associative power, mixed operations

#### Rational Functions
- num, den, frac
- rational, float

### 6. Error and Internal Tests (`06_error_internal_tests.rs` - 46 tests)

#### Error Types
- CalcError display, to string, string to error

#### Division by Zero Errors
- Division, modulo, nested expressions

#### Domain Errors
- Negative square root, negative logarithm
- asin/acosh/atanh out of range

#### Invalid Input
- Empty expressions, whitespace, operators only
- Invalid numbers

#### Syntax Errors
- Consecutive operators, mismatched parentheses
- Empty parentheses

#### Function Argument Errors
- No arguments, too many arguments, wrong argument count
- Invalid functions

#### Bitwise Errors
- Subtraction not supported, invalid shift

#### Factorial Errors
- Negative, too large, non-integer

#### Value Type Tests
- from_int, from_float
- add, div, pow
- Division by zero errors

#### Lexical Analysis Tests
- Simple expressions, prefix numbers
- Negative prefix numbers

### 7. User-Defined Tests (`07_user_defined_tests.rs` - 8 tests)

#### Built-in Constants
- C_PI, C_E
- Constant expressions, constants with functions
- Constants with comparison operations

### 8. Extended Tests (`08_extended_tests.rs` - 76 tests)

#### Advanced Arithmetic
- Complex expressions, power of power, right-associative power
- Factorial expressions, modulo with negative

#### Advanced Functions
- Nested functions, trigonometric identities
- Logarithm properties, sum/prod functions
- abs/floor/ceil/round

#### BigInt Extended
- bfactorial(100), bpow(2, 200)
- comb(52,5) poker combinations
- perm, gcd coprime, lcm coprime
- isprime prime/composite, nextprime

#### Number Format Extended
- Binary/octal/hexadecimal operations
- Mixed formats, format output calculations

#### Scientific Notation Extended
- Large numbers, small numbers, multiplication, division
- Negative exponents

#### REPL Extended
- Mode switching, history result chains
- hex/oct/bin conversion, empty line help
- Tab completion, create and use functions/constants
- quit command

#### Error Handling Extended
- Mismatched parentheses, empty parentheses
- Consecutive operators, operators only
- Invalid functions, wrong argument count
- sqrt/log/asin errors
- factorial errors, bitwise errors

#### Help System Extended
- Help flags, function/operator/format/constant help
- Subcommands, info command

#### Edge Cases Extended
- Very large multiplication, zero power
- 0!/1!, decimal precision
- Multiple decimals, whitespace handling
- Single numbers, negative numbers
- Floating point comparison

---

## Running Tests

```bash
# Run all tests
cargo test

# Run specific category tests
cargo test --test 01_core_tests
cargo test --test 02_functions_tests
cargo test --test 03_cli_formats_tests
cargo test --test 04_repl_tests
cargo test --test 05_rational_tests
cargo test --test 06_error_internal_tests
cargo test --test 07_user_defined_tests
cargo test --test 08_extended_tests

# Run single test
cargo test test_addition
cargo test test_sin
cargo test test_repl_addition

# Run specific mode tests
cargo test cli           # CLI related tests
cargo test repl          # REPL related tests
cargo test rational      # Rational related tests
cargo test error         # Error related tests
```

---

## Test Coverage

| Module | Test Files | Tests | Coverage |
|----------|----------|--------|--------|
| Arithmetic | 01, 08 | ~80 | ✅ |
| Bitwise | 01, 08 | ~20 | ✅ |
| Comparison | 01, 04, 08 | ~25 | ✅ |
| Math Functions | 02, 08 | ~50 | ✅ |
| BigInt Functions | 02, 04, 08 | ~25 | ✅ |
| CLI Arguments | 03, 08 | ~40 | ✅ |
| Number Formats | 03, 08 | ~25 | ✅ |
| REPL Interaction | 04, 08 | ~80 | ✅ |
| Rational | 05 | ~33 | ✅ |
| Parser | 05 | ~20 | ✅ |
| Error Handling | 06, 08 | ~50 | ✅ |
| Internal Modules | 06 | ~15 | ✅ |
| Constants | 01, 07 | ~15 | ✅ |
| Help System | 03, 08 | ~15 | ✅ |
| **Total** | **8 files** | **~373** | **✅** |

---

## Optimization Notes

**Test File Organization:**
1. Grouped by functional domain (core, functions, CLI, REPL, rational, error, user-defined, extended)
2. Use numeric prefixes to maintain file order
3. Each file contains related subcategory tests
4. Extended test file contains cross-category comprehensive tests

**Test Coverage Strategy:**
- Basic functionality tests
- Edge case tests
- Error handling tests
- Combined functionality tests
- CLI and REPL dual mode tests
