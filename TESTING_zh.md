# Ralqlator 测试指南

> **其他语言版本**: [English](TESTING_en.md)

## 测试类型

本项目包含五种类型的测试：

1. **单元测试** - 测试单个函数和模块
2. **集成测试** - 测试模块间的交互
3. **端到端测试** - 测试完整的 CLI 和 REPL 流程
4. **基准测试** - 性能基准测试
5. **模糊测试** - 安全性模糊测试

## 运行测试

### 运行所有测试

```bash
# 运行所有测试
cargo test

# 运行所有测试（包含输出）
cargo test -- --nocapture
```

### 运行特定测试类别

```bash
# 核心功能测试
cargo test --test 01_core_tests

# 函数测试
cargo test --test 02_functions_tests

# CLI 和格式测试
cargo test --test 03_cli_formats_tests

# REPL 测试
cargo test --test 04_repl_tests

# 有理数测试
cargo test --test 05_rational_tests

# 错误处理测试
cargo test --test 06_error_internal_tests

# 用户定义测试
cargo test --test 07_user_defined_tests

# 扩展测试
cargo test --test 08_extended_tests

# 端到端集成测试
cargo test --test e2e_integration_tests
```

### 运行单个测试

```bash
# 运行特定测试函数
cargo test test_addition
cargo test test_sin
cargo test test_repl_addition

# 运行包含关键词的测试
cargo test rational
cargo test error
cargo test cli
```

### 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试组
cargo bench --bench calculator_bench -- parsing_benches
cargo bench --bench calculator_bench -- evaluation_benches
cargo bench --bench calculator_bench -- bigint_benches

# 运行单个基准测试
cargo bench --bench calculator_bench -- bench_parse_simple
cargo bench --bench calculator_bench -- bench_eval_arithmetic
```

### 运行模糊测试

```bash
# 安装 cargo-fuzz（如果未安装）
cargo install cargo-fuzz

# 运行模糊测试
cd fuzz
cargo fuzz run parse_expression

# 运行模糊测试（带超时）
cargo fuzz run parse_expression -- -max_total_time=60

# 查看语料库
ls fuzz/corpus/parse_expression/
```

## 测试覆盖范围

### 单元测试和集成测试 (~363 个测试)

| 类别 | 文件 | 测试数 |
|------|------|--------|
| 核心功能 | 01_core_tests.rs | 66 |
| 数学函数 | 02_functions_tests.rs | 35 |
| CLI 格式 | 03_cli_formats_tests.rs | 33 |
| REPL 交互 | 04_repl_tests.rs | 55 |
| 有理数 | 05_rational_tests.rs | 44 |
| 错误处理 | 06_error_internal_tests.rs | 46 |
| 用户定义 | 07_user_defined_tests.rs | 8 |
| 扩展测试 | 08_extended_tests.rs | 76 |

### 端到端测试 (34 个测试)

| 类别 | 测试内容 |
|------|----------|
| 完整工作流 | 算术、函数、位运算、格式转换 |
| REPL 会话 | 函数定义、常量定义、模式切换 |
| 错误处理 | 除零、无效表达式、定义域错误 |
| 性能测试 | 大数计算、复杂表达式 |
| 边界情况 | 长表达式、嵌套括号、混合格式 |
| 帮助系统 | CLI 帮助、REPL 帮助 |
| 回归测试 | 历史问题验证 |

### 基准测试 (24 个基准)

| 类别 | 基准测试 |
|------|----------|
| 解析 | 简单表达式、复杂表达式、函数调用、嵌套函数、科学记数法、十六进制输入 |
| 求值 | 算术运算、指数运算、阶乘、三角函数、对数、比较运算 |
| BigInt | GCD、LCM、bfactorial、bpow、isprime |
| 参数化 | 幂运算 (10/50/100/500)、阶乘 (5/10/20/50)、大阶乘 (50/100/500/1000) |

## 测试最佳实践

### 编写新测试

1. **命名规范**
   ```rust
   #[test]
   fn test_feature_scenario() {
       // 测试代码
   }
   ```

2. **测试结构**
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

3. **错误测试**
   ```rust
   #[test]
   fn test_division_by_zero() {
       let (_, stderr, success) = run_cli(&["1 / 0"]);
       assert!(!success);
       assert!(stderr.contains("Error"));
   }
   ```

### 基准测试指南

1. **使用 black_box**
   ```rust
   use criterion::black_box;
   
   c.bench_function("benchmark", |b| {
       b.iter(|| function(black_box(input)))
   });
   ```

2. **参数化基准**
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

### 模糊测试指南

1. **处理无效输入**
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

2. **分析崩溃**
   ```bash
   # 查看崩溃输入
   cargo fuzz crash parse_expression fuzz/artifacts/parse_expression/crash-xxx
   ```

## 持续集成

### GitHub Actions 配置

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

## 测试覆盖率

### 生成覆盖率报告

```bash
# 安装 cargo-tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 查看报告
open tarpaulin-report.html
```

### 覆盖率目标

- **核心模块**: >90%
- **错误处理**: >85%
- **用户界面**: >80%
- **整体**: >85%

## 故障排查

### 测试失败

```bash
# 查看详细错误信息
cargo test -- --nocapture

# 运行单个失败测试
cargo test test_name -- --nocapture

# 使用 backtrace
RUST_BACKTRACE=1 cargo test test_name
```

### 基准测试不稳定

```bash
# 增加采样次数
cargo bench -- --sample-size 100

# 禁用优化（调试用）
CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo bench
```

### 模糊测试问题

```bash
# 重置语料库
rm -rf fuzz/corpus/*

# 使用现有语料库
cargo fuzz run parse_expression fuzz/corpus/seed/
```

## 性能优化建议

基于基准测试结果：

1. **解析优化**
   - 减少字符串克隆
   - 使用字符串切片
   - 预分配向量容量

2. **求值优化**
   - 缓存常用计算结果
   - 使用迭代代替递归
   - 内联小函数

3. **BigInt 优化**
   - 使用更高效的算法
   - 减少内存分配
   - 批量操作

## 贡献指南

1. **新功能的测试**
   - 至少 3 个单元测试
   - 1 个集成测试
   - 1 个端到端测试

2. **Bug 修复的测试**
   - 回归测试
   - 边界条件测试

3. **性能优化的测试**
   - 基准测试对比
   - 性能回归测试

## 资源链接

- [Rust 测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion 基准测试](https://bheisler.github.io/criterion.rs/book/)
- [cargo-fuzz 模糊测试](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [cargo-tarpaulin 覆盖率](https://github.com/xd009642/tarpaulin)
