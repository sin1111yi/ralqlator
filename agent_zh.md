# Ralqlator - 命令行计算器

## 项目概述

Ralqlator 是一个功能丰富的命令行计算器，支持标准算术运算、位运算、多种数字格式和数学函数。

> **AI 生成项目**: 本项目完全由 **Qwen3.5-plus**（阿里巴巴通义千问大语言模型）生成。

## 项目结构

```
raculator/
├── Cargo.toml              # 项目配置和依赖
└── src/
    ├── main.rs            # 程序入口，CLI 解析和调度
    ├── lib.rs             # 测试模块
    ├── cli.rs             # 命令行参数定义 (clap)
    ├── repl.rs            # 交互模式 (REPL)
    ├── calculator.rs      # 计算核心编排
    ├── evaluator.rs       # 后缀表达式求值
    ├── functions.rs       # 数学函数具体实现
    ├── operator.rs        # 操作符/函数识别和优先级
    ├── shunting_yard.rs   # 中缀转后缀算法
    └── token.rs           # 字符串分词
```

## 核心模块说明

### main.rs
- **职责**: 程序入口，解析 CLI 参数，分发到不同模式
- **关键函数**:
  - `main()`: 解析参数并调度到计算模式或交互模式
  - `print_help()`: 显示帮助信息
  - `print_result()`: 格式化输出标准模式结果
  - `print_bitwise_result()`: 格式化输出位运算结果

### cli.rs
- **职责**: 命令行参数定义
- **参数**:
  - `expression`: 位置参数，直接计算的表达式
  - `-r, --row`: 表达式参数（简写）
  - `-x, --hex`: 十六进制输出
  - `-o, --oct`: 八进制输出
  - `-b, --bin`: 二进制输出
  - `-B, --bits`: 位运算模式
  - `-F, --functions`: 显示支持的函数

### calculator.rs
- **职责**: 计算流程编排
- **函数**:
  - `calculate()`: 标准模式计算（f64）
  - `calculate_bitwise()`: 位运算模式计算（i64）
- **流程**: 分词 → 常数解析 → 中缀转后缀 → 求值

### token.rs
- **职责**: 词法分析，将输入字符串分割为 token
- **支持**:
  - 科学记数法 (1e3, 2.5e-3)
  - 数字前缀：0b(二进制), 0o(八进制), 0x(十六进制)
  - 常数：pi, e
  - 位运算符 (位运算模式)
- **关键函数**:
  - `tokenize()`: 分词主函数
  - `resolve_constants()`: 解析常数和前缀数字

### shunting_yard.rs
- **职责**: 中缀表达式转后缀表达式（调度场算法）
- **函数**:
  - `infix_to_postfix()`: 标准模式转换
  - `infix_to_postfix_bitwise()`: 位运算模式转换

### evaluator.rs
- **职责**: 后缀表达式求值
- **函数**:
  - `eval_postfix()`: 标准模式求值（f64）
  - `eval_postfix_bitwise()`: 位运算模式求值（i64）

### functions.rs
- **职责**: 数学函数实现
- **函数列表**:
  - `eval_log_base()`: 自定义底对数
  - `eval_lg()`: 以 10 为底对数
  - `eval_ln()`: 自然对数
  - `eval_sqrt()`: 平方根
  - `eval_pow()`: 幂函数
  - `eval_sin/cos/tan()`: 三角函数
  - `eval_asin/acos/atan()`: 反三角函数
  - `eval_mod()`: 取余函数

### repl.rs
- **职责**: 交互式计算器
- **功能**:
  - 历史输入（上下键浏览）
  - @ 插入上次结果
  - hex/oct/bin 命令转换格式
  - Tab 键命令补全（空行时显示帮助）
  - `mode` 命令切换标准模式和位运算模式
  - 模式相关帮助显示
- **结构**:
  - `run_repl()`: 标准模式
  - `run_repl_bitwise()`: 位运算模式
  - `run_repl_with_mode()`: 统一的 REPL 支持模式切换
- **关键命令**:
  - `Tab`: 命令补全（空行时显示帮助）
  - `@`: 插入上次结果
  - `mode`: 切换标准模式和位运算模式
  - `mode standard/std/s`: 切换到标准模式
  - `mode bitwise/bit/b`: 切换到位运算模式
  - `help [mode]`: 显示帮助（可选指定模式）
  - `operators [mode]`: 显示当前/指定模式的运算符
  - `functions [mode]`: 显示函数帮助
  - `q/quit`: 退出

### operator.rs
- **职责**: 操作符和函数识别
- **函数**:
  - `is_operator()`: 判断是否为运算符
  - `is_bitwise_operator()`: 判断是否为位运算符
  - `is_function()`: 判断是否为函数
  - `precedence()`: 运算符优先级
  - `bitwise_precedence()`: 位运算符优先级

## 功能特性

### 标准运算符
| 运算符 | 说明 | 优先级 |
|--------|------|--------|
| `+` | 加法 | 1 |
| `-` | 减法 | 1 |
| `*` | 乘法 | 2 |
| `/` | 除法 | 2 |
| `%` | 取余 | 2 |
| `^` | 幂运算 | 3 |

### 位运算符（-B 模式）
| 运算符 | 说明 | 优先级 |
|--------|------|--------|
| `\|` | 按位或 | 1 |
| `^` | 按位异或 | 2 |
| `&` | 按位与 | 3 |
| `<<` | 左移 | 4 |
| `>>` | 右移 | 4 |
| `~` | 按位取反 | 5 |

### 数学函数
| 函数 | 说明 | 参数 |
|------|------|------|
| `lg(x)` | 以 10 为底对数 | 1 或 2 |
| `lg(x, base)` | 自定义底对数 | 2 |
| `log(x, base)` | 自定义底对数 | 2 |
| `ln(x)` | 自然对数 | 1 |
| `sqrt(x)` | 平方根 | 1 |
| `pow(x, y)` | 幂函数 | 2 |
| `sin(x)` | 正弦 | 1 |
| `cos(x)` | 余弦 | 1 |
| `tan(x)` | 正切 | 1 |
| `asin(x)` | 反正弦 | 1 |
| `acos(x)` | 反余弦 | 1 |
| `atan(x)` | 反正切 | 1 |
| `mod(a, b)` | 取余函数 | 2 |

### 常数
- `pi` / `PI`: 圆周率 ≈ 3.14159
- `e` / `E`: 自然常数 ≈ 2.71828

### 数字格式
- 十进制：`123`, `-456`, `3.14`
- 二进制：`0b1010`, `-0b1100`
- 八进制：`0o755`, `-0o123`
- 十六进制：`0xFF`, `-0x1A`
- 科学记数法：`1e3`, `2.5e-3`, `1.23E+10`

## 使用示例

### 命令行模式
```bash
# 基本计算
cargo run -- "1 + 2 * 3"           # 输出：7

# 使用函数
cargo run -- "lg(100)"             # 输出：2
cargo run -- "sin(pi / 2)"         # 输出：1

# 位运算
cargo run -- -B "12 & 10"          # 输出：8
cargo run -- -Bx "0xFF & 0x0F"     # 输出：0xF

# 进制转换
cargo run -- -x "255"              # 输出：0xFF
cargo run -- -o "255"              # 输出：0o377
cargo run -- -b "255"              # 输出：0b11111111

# 查看帮助
cargo run -- -F
cargo run -- --help
```

### 交互模式
```bash
cargo run
```

交互模式命令：
- `Tab`: 命令补全（空行时显示帮助）
- `q` / `quit`: 退出
- `@`: 插入上次结果
- `hex`: 显示上次结果的十六进制
- `oct`: 显示上次结果的八进制
- `bin`: 显示上次结果的二进制
- `mode`: 切换标准模式和位运算模式
- `mode standard`: 切换到标准模式
- `mode bitwise`: 切换到位运算模式
- `help`: 显示完整帮助信息
- `help standard`: 显示标准模式帮助
- `help bitwise`: 显示位运算模式帮助
- `operators`: 显示当前模式运算符
- `functions`: 显示数学函数
- `formats`: 显示数字格式
- `constants`: 显示数学常数

位运算交互模式：
```bash
cargo run -- -B
```

示例会话：
```
> 255
255
> hex
0xFF
> @ + 1
256
> mode
Switched to Bitwise mode (integer operations)
> 12 & 10
8
> quit
```

## 数据流

```
输入字符串
    ↓
tokenize() → Vec<String>
    ↓
resolve_constants() → Vec<String>
    ↓
infix_to_postfix() → Vec<String> (后缀表达式)
    ↓
eval_postfix() → Result<T, String>
    ↓
输出结果
```

## 测试

运行所有测试：
```bash
cargo test
```

测试覆盖：
- 基本算术运算 (9 个测试)
- 数字格式 (6 个测试)
- 常数 (3 个测试)
- 函数计算 (12 个测试)
- 三角函数 (6 个测试)
- 位运算 (8 个测试)
- 错误处理 (7 个测试)
- 词法分析 (6 个测试)
- 语法分析 (3 个测试)
- 输出格式 (3 个测试)
- 集成测试 (5 个测试)

**总计：59 个测试用例**

## 依赖

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # CLI 参数解析
rustyline = "14.0"                                  # 交互式输入
```

## 性能优化

1. **减少克隆**: 使用 `std::mem::take()` 替代 `clone()` + `clear()`
2. **模式匹配**: 使用 `match` 替代多层 `if-else`
3. **提前返回**: 减少不必要的条件判断
4. **函数提取**: 提高代码复用性和可读性

## 代码规范

- 所有代码通过 `cargo fmt` 格式化
- 所有代码通过 `cargo clippy` 检查
- 使用英文注释和文档
- 函数命名清晰表达意图
- 模块职责单一明确

## 扩展建议

1. **添加更多函数**: 在 `functions.rs` 中添加新函数，在 `operator.rs` 中注册
2. **支持更多进制**: 修改 `token.rs` 的 `parse_prefixed_number()`
3. **添加变量支持**: 扩展 `token.rs` 和 `evaluator.rs`
4. **表达式历史**: 在 `repl.rs` 中添加历史记录功能
5. **自定义精度**: 添加浮点数精度控制选项

## 常见问题

**Q: 位运算为什么使用 i64？**
A: i64 提供足够的位宽，同时支持负数的位运算。

**Q: 如何添加新函数？**
A:
1. 在 `functions.rs` 中实现函数
2. 在 `operator.rs` 的 `is_function()` 中添加函数名
3. 在 `evaluator.rs` 的 `eval_postfix()` 中添加处理逻辑
4. 在 `main.rs` 的帮助信息中添加说明

## 作者

开发于 2026 年
