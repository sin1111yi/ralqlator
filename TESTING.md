# Ralqlator Testing Guide

> **Other Languages**: [中文](TESTING_zh.md)

## Test Types

This project contains four types of tests:

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test interactions between modules
3. **End-to-End Tests** - Test complete CLI and REPL workflows
4. **Benchmark Tests** - Performance benchmarks
5. **Fuzz Tests** - Security fuzz testing

## Running Tests

### Run All Tests

```bash
# Run all tests
cargo test

# Run all tests (with output)
cargo test -- --nocapture
```

### Run Specific Test Categories

```bash
# Core functionality tests
cargo test --test 01_core_tests

# Function tests
cargo test --test 02_functions_tests

# CLI and format tests
cargo test --test 03_cli_formats_tests

# REPL tests
cargo test --test 04_repl_tests

# Rational tests
cargo test --test 05_rational_tests

# Error handling tests
cargo test --test 06_error_internal_tests

# User-defined tests
cargo test --test 07_user_defined_tests

# Extended tests
cargo test --test 08_extended_tests

# End-to-end integration tests
cargo test --test e2e_integration_tests
```

### Run Single Test

```bash
# Run specific test function
cargo test test_addition
cargo test test_sin
cargo test test_repl_addition

# Run tests containing keyword
cargo test rational
cargo test error
cargo test cli
```

### Run Benchmark Tests

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark groups
cargo bench --bench calculator_bench -- parsing_benches
cargo bench --bench calculator_bench -- evaluation_benches
cargo bench --bench calculator_bench -- bigint_benches

# Run single benchmark
cargo bench --bench calculator_bench -- bench_parse_simple
cargo bench --bench calculator_bench -- bench_eval_arithmetic
```

### Run Fuzz Tests

```bash
# Install cargo-fuzz (if not installed)
cargo install cargo-fuzz

# Run fuzz tests
cd fuzz
cargo fuzz run parse_expression

# Run fuzz tests (with timeout)
cargo fuzz run parse_expression -- -max_total_time=60

# View corpus
ls fuzz/corpus/parse_expression/
```

## Test Coverage

### Unit and Integration Tests (~363 tests)

| Category | File | Tests |
|------|------|--------|
| Core Functionality | 01_core_tests.rs | 66 |
| Math Functions | 02_functions_tests.rs | 35 |
| CLI Formats | 03_cli_formats_tests.rs | 33 |
| REPL Interaction | 04_repl_tests.rs | 55 |
| Rational | 05_rational_tests.rs | 44 |
| Error Handling | 06_error_internal_tests.rs | 46 |
| User-Defined | 07_user_defined_tests.rs | 8 |
| Extended Tests | 08_extended_tests.rs | 76 |

### End-to-End Tests (34 tests)

| Category | Test Content |
|------|----------|
| Complete Workflows | Arithmetic, functions, bitwise, format conversion |
| REPL Sessions | Function definition, constant definition, mode switching |
| Error Handling | Division by zero, invalid expressions, domain errors |
| Performance Tests | Large number calculations, complex expressions |
| Edge Cases | Long expressions, nested parentheses, mixed formats |
| Help System | CLI help, REPL help |
| Regression Tests | Historical issue verification |

### Benchmark Tests (24 benchmarks)

| Category | Benchmarks |
|------|----------|
| Parsing | Simple expressions, complex expressions, function calls, nested functions, scientific notation, hex input |
| Evaluation | Arithmetic, exponentiation, factorial, trigonometric, logarithms, comparison |
| BigInt | GCD, LCM, bfactorial, bpow, isprime |
| Parametric | Power (10/50/100/500), factorial (5/10/20/50), bfactorial (50/100/500/1000) |

## Testing Best Practices

### Writing New Tests

1. **Naming Convention**
   ```rust
   #[test]
   fn test_feature_scenario() {
       // Test code
   }
   ```

2. **Test Structure**
   ```rust
   #[test]
   fn test_addition() {
       // Arrange
       let (stdout, _, success) = run_cli(&["1 + 2"]);
       
       // Act & Assert
       assert!(success);
       assert!(stdout.contains("3"));
   }
   ```

3. **Error Tests**
   ```rust
   #[test]
   fn test_division_by_zero() {
       let (_, stderr, success) = run_cli(&["1 / 0"]);
       assert!(!success);
       assert!(stderr.contains("Error"));
   }
   ```

### Benchmark Guidelines

1. **Use black_box**
   ```rust
   use criterion::black_box;
   
   c.bench_function("benchmark", |b| {
       b.iter(|| function(black_box(input)))
   });
   ```

2. **Parametric Benchmarks**
   ```rust
   let mut group = c.benchmark_group("group_name");
   for param in [10, 100, 1000] {
       group.bench_with_input(
           BenchmarkId::from_parameter(param),
           &param,
           |b, &param| {
               b.iter(|| function(param))
           },
       );
   }
   ```

### Fuzz Testing Guidelines

1. **Handle Invalid Input**
   ```rust
   fuzz_target!(|data: &[u8]| {
       if let Ok(input) = std::str::from_utf8(data) {
           if input.is_empty() || input.len() > 1000 {
               return;
           }
           let _ = parse_expression(input);
       }
   });
   ```

2. **Analyze Crashes**
   ```bash
   # View crash input
   cargo fuzz crash parse_expression fuzz/artifacts/parse_expression/crash-xxx
   ```

## Continuous Integration

### GitHub Actions Configuration

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --verbose
      
      - name: Run Clippy
        run: cargo clippy -- -D warnings
      
      - name: Run benchmarks
        run: cargo bench
```

## Test Coverage

### Generate Coverage Report

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View report
open tarpaulin-report.html
```

### Coverage Targets

- **Core Modules**: >90%
- **Error Handling**: >85%
- **User Interface**: >80%
- **Overall**: >85%

## Troubleshooting

### Test Failures

```bash
# View detailed error messages
cargo test -- --nocapture

# Run single failing test
cargo test test_name -- --nocapture

# Use backtrace
RUST_BACKTRACE=1 cargo test test_name
```

### Unstable Benchmarks

```bash
# Increase sample size
cargo bench -- --sample-size 100

# Disable optimization (for debugging)
CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo bench
```

### Fuzz Testing Issues

```bash
# Reset corpus
rm -rf fuzz/corpus/*

# Use existing corpus
cargo fuzz run parse_expression fuzz/corpus/seed/
```

## Performance Optimization Suggestions

Based on benchmark results:

1. **Parsing Optimization**
   - Reduce string cloning
   - Use string slices
   - Pre-allocate vector capacity

2. **Evaluation Optimization**
   - Cache frequently computed results
   - Use iteration instead of recursion
   - Inline small functions

3. **BigInt Optimization**
   - Use more efficient algorithms
   - Reduce memory allocations
   - Batch operations

## Contribution Guidelines

1. **Tests for New Features**
   - At least 3 unit tests
   - 1 integration test
   - 1 end-to-end test

2. **Tests for Bug Fixes**
   - Regression tests
   - Edge case tests

3. **Tests for Performance Optimizations**
   - Benchmark comparisons
   - Performance regression tests

## Resource Links

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion Benchmarking](https://bheisler.github.io/criterion.rs/book/)
- [cargo-fuzz Fuzz Testing](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [cargo-tarpaulin Coverage](https://github.com/xd009642/tarpaulin)
