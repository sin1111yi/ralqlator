# Ralqlator

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Build Status](https://img.shields.io/badge/test-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-GPLV3-blue)]()
[![Rust](https://img.shields.io/badge/rust-2026-orange)]()

> **AI 生成项目**: 本项目完全由 **qwen3.5-plus**（阿里巴巴通义千问大语言模型）生成。

一个用 Rust 编写的强大命令行计算器。

## 功能特性

- **基本算术**: 加法、减法、乘法、除法、取余、幂运算
- **位运算**: 与、或、异或、非、左移、右移
- **数学函数**: 对数、三角函数、平方根、幂函数
- **多种数字格式**: 十进制、二进制 (0b)、八进制 (0o)、十六进制 (0x)
- **科学记数法**: 支持 1e3、2.5e-3 等格式
- **输出格式转换**: 以十六进制、八进制或二进制显示结果
- **交互式 REPL**: 支持历史导航和上次结果插入
- **数学常数**: π (圆周率) 和 e (自然常数)

## 安装

### 从源码安装

```bash
git clone <repository-url>
cd ralqlator
cargo build --release
```

二进制文件位于 `target/release/ralqlator`。

### 使用 Cargo 安装

```bash
cargo install ralqlator
```

## 使用方法

### 命令行模式

```bash
# 基本计算
ralqlator "1 + 2 * 3"              # 输出：7

# 使用函数
ralqlator "lg(100)"                # 输出：2
ralqlator "sin(pi / 2)"            # 输出：1

# 位运算
ralqlator -B "12 & 10"             # 输出：8
ralqlator -B "0xFF & 0x0F"         # 输出：15

# 输出格式转换
ralqlator -x "255"                 # 输出：0xFF
ralqlator -o "255"                 # 输出：0o377
ralqlator -b "255"                 # 输出：0b11111111

# 组合选项
ralqlator -Bx "0xFF & 0x0F"        # 输出：0xF
```

### 交互模式

```bash
ralqlator
```

交互模式下：
- 输入表达式并按 Enter 计算
- 使用 `@` 或 `Alt+G` 插入上次结果
- 使用 `hex`、`oct`、`bin` 将上次结果转换为不同格式
- 输入 `q` 或 `quit` 退出

```
> 255
= 255
> hex
= 0xFF
> @ + 1
= 256
> quit
```

### 位运算交互模式

```bash
ralqlator -B
```

## 运算符

### 标准运算符

| 运算符 | 说明 | 示例 | 结果 |
|--------|------|------|------|
| `+` | 加法 | `1 + 2` | 3 |
| `-` | 减法 | `5 - 3` | 2 |
| `*` | 乘法 | `3 * 4` | 12 |
| `/` | 除法 | `10 / 2` | 5 |
| `%` | 取余 | `10 % 3` | 1 |
| `^` | 幂运算 | `2 ^ 3` | 8 |

### 位运算符（使用 `-B` 标志）

| 运算符 | 说明 | 示例 | 结果 |
|--------|------|------|------|
| `&` | 按位与 | `12 & 10` | 8 |
| `\|` | 按位或 | `12 \| 10` | 14 |
| `^` | 按位异或 | `12 ^ 10` | 6 |
| `~` | 按位非 | `~0` | -1 |
| `<<` | 左移 | `8 << 2` | 32 |
| `>>` | 右移 | `8 >> 2` | 2 |

## 函数

| 函数 | 说明 | 示例 | 结果 |
|------|------|------|------|
| `lg(x)` | 以 10 为底对数 | `lg(100)` | 2 |
| `lg(x, base)` | 自定义底对数 | `lg(8, 2)` | 3 |
| `log(x, base)` | 自定义底对数 | `log(27, 3)` | 3 |
| `ln(x)` | 自然对数 | `ln(e)` | 1 |
| `sqrt(x)` | 平方根 | `sqrt(16)` | 4 |
| `pow(x, y)` | 幂函数 | `pow(2, 10)` | 1024 |
| `sin(x)` | 正弦（弧度） | `sin(0)` | 0 |
| `cos(x)` | 余弦（弧度） | `cos(0)` | 1 |
| `tan(x)` | 正切（弧度） | `tan(0)` | 0 |
| `asin(x)` | 反正弦 | `asin(1)` | π/2 |
| `acos(x)` | 反余弦 | `acos(1)` | 0 |
| `atan(x)` | 反正切 | `atan(1)` | π/4 |
| `mod(a, b)` | 取余函数 | `mod(10, 3)` | 1 |

## 常数

| 常数 | 值 | 示例 |
|------|-----|------|
| `pi`, `PI` | ≈ 3.14159 | `sin(pi / 2)` = 1 |
| `e`, `E` | ≈ 2.71828 | `ln(e)` = 1 |

## 数字格式

### 输入格式

| 格式 | 前缀 | 示例 | 值 |
|------|------|------|-----|
| 十进制 | 无 | `255` | 255 |
| 二进制 | `0b` | `0b11111111` | 255 |
| 八进制 | `0o` | `0o377` | 255 |
| 十六进制 | `0x` | `0xFF` | 255 |

### 输出格式选项

| 选项 | 说明 | 示例 |
|------|------|------|
| `-x`, `--hex` | 十六进制输出 | `ralqlator -x "255"` → `0xFF` |
| `-o`, `--oct` | 八进制输出 | `ralqlator -o "255"` → `0o377` |
| `-b`, `--bin` | 二进制输出 | `ralqlator -b "255"` → `0b11111111` |

## 命令行选项

```
Usage: ralqlator [OPTIONS] [EXPR] [COMMAND]

Arguments:
  [EXPR]  要计算的表达式

Options:
  -r, --row <EXPR>     要计算的表达式（简写）
  -x, --hex            以十六进制格式输出结果
  -o, --oct            以八进制格式输出结果
  -b, --bin            以二进制格式输出结果
  -B, --bits           位运算模式
  -F, --functions      显示函数帮助
  -O, --operators      显示运算符帮助
  -N, --formats        显示数字格式帮助
  -C, --constants      显示常数帮助
  -h, --help           打印帮助信息

Commands:
  functions            显示数学函数
  operators            显示运算符
  formats              显示数字格式
  constants            显示数学常数
  info                 显示所有帮助信息
```

### 帮助示例

```bash
# 显示函数帮助
ralqlator functions
ralqlator -F

# 显示运算符帮助
ralqlator operators
ralqlator -O

# 显示数字格式帮助
ralqlator formats
ralqlator -N

# 显示常数帮助
ralqlator constants
ralqlator -C

# 显示所有帮助
ralqlator info
```

## 测试

运行所有测试：

```bash
cargo test
```

测试套件包含 61 个测试用例，覆盖：
- 基本算术运算
- 数字格式解析
- 数学常数
- 函数计算
- 三角函数
- 位运算
- 错误处理
- 词法分析和语法分析

## 项目结构

```
ralqlator/
├── Cargo.toml              # 项目配置
├── agent.md                # AI 助手英文文档
├── agent_zh.md             # AI 助手中文文档
└── src/
    ├── main.rs            # 程序入口
    ├── lib.rs             # 测试模块
    ├── cli.rs             # CLI 参数定义
    ├── repl.rs            # 交互式 REPL
    ├── calculator.rs      # 计算编排
    ├── evaluator.rs       # 表达式求值
    ├── functions.rs       # 数学函数
    ├── operator.rs        # 运算符定义
    ├── shunting_yard.rs   # 中缀转后缀算法
    ├── token.rs           # 词法分析
    └── linked_list.rs     # 链表数据结构
```

## 示例

```bash
# 复杂表达式
ralqlator "sqrt(pow(3, 2) + pow(4, 2))"    # 输出：5

# 科学记数法
ralqlator "1e3 + 2.5e-3"                   # 输出：1000.0025

# 混合格式
ralqlator "0xFF + 0b1010"                  # 输出：265

# 三角恒等式
ralqlator "sin(pi / 2) + cos(0)"           # 输出：2

# 对数
ralqlator "lg(1000)"                       # 输出：3
ralqlator "log(8, 2)"                      # 输出：3

# 位运算带格式输出
ralqlator -Bb "8 << 2"                     # 输出：0b100000
```

## 许可证

MIT 许可证 - 详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 致谢

- 使用 [Rust](https://www.rust-lang.org/) 构建
- CLI 解析使用 [clap](https://github.com/clap-rs/clap)
- 交互式输入使用 [rustyline](https://github.com/kkawakam/rustyline)
