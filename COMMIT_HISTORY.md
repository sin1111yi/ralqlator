# Raculator Commit History

## Current Git Log

```
d2d0a00 docs: add git commit history reference
2ba9bee feat: initialize Rust project with cargo
```

## Full Commit Messages

### Commit 1: Project Initialization
```
feat: initialize Rust project with cargo

- Create new Rust project structure
- Set up Cargo.toml with basic configuration
- Add all source modules:
  - main.rs: Program entry point
  - calculator.rs: Calculation orchestration
  - evaluator.rs: Expression evaluation
  - functions.rs: Mathematical functions
  - operator.rs: Operator definitions
  - shunting_yard.rs: Infix to postfix conversion
  - token.rs: Lexical analysis
  - linked_list.rs: Linked list data structure
  - cli.rs: CLI argument definitions
  - repl.rs: Interactive REPL
  - lib.rs: Test module
```

### Commit 2: Documentation
```
docs: add git commit history reference

- Add GIT_COMMITS.md with recommended commit sequence
- Document 20 feature commits
- Include file change tracking per commit
```

## Recommended Future Commits

For future development, follow the conventional commit format:

```
feat: <new feature>
fix: <bug fix>
docs: <documentation>
refactor: <code refactoring>
test: <tests>
chore: <maintenance>
```

## Feature Summary

The initial commit includes all core features:

| Category | Features |
|----------|----------|
| **Operators** | +, -, *, /, %, ^ |
| **Bitwise** | &, \|, ^, ~, <<, >> (-B mode) |
| **Functions** | lg, log, ln, sqrt, pow, sin, cos, tan, asin, acos, atan, mod |
| **Constants** | pi, e |
| **Input Formats** | Decimal, Binary (0b), Octal (0o), Hex (0x), Scientific (1e3) |
| **Output Formats** | Decimal, Hex (-x), Octal (-o), Binary (-b) |
| **CLI** | -r, -x, -o, -b, -B, -F, -h |
| **REPL** | History, Alt+G, @, hex/oct/bin commands |
| **Tests** | 61 test cases |
