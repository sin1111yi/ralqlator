# Ralqlator

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Build Status](https://img.shields.io/badge/test-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-GPLv3-blue)]()
[![Rust](https://img.shields.io/badge/rust-2021-orange)]()

> **AI 生成项目**: 本项目完全由 **qwen3.5-plus**（阿里巴巴通义千问大语言模型）在 **qwen-coder**（阿里巴巴 AI agent）下生成。

一个用 Rust 编写的强大命令行计算器。

## 功能特性

- **基本算术**: 加法、减法、乘法、除法、取余、幂运算
- **位运算**: 与、或、异或、非、左移、右移
- **数学函数**: 对数、三角函数、平方根、幂函数、阶乘
- **多参数函数**: `sum(a,b,...)`, `prod(a,b,...)` 用于多参数求和和连乘
- **数列运算**: `suma(seq, begin, end)` 用于数列求和
- **用户定义函数**: 使用 `create func name(args) = expression` 创建自定义函数
- **用户定义数列**: 使用 `create seq name(n) = formula` 创建数列
- **用户定义常量**: 使用 `create const NAME value` 创建常量
- **多种数字格式**: 十进制、二进制 (0b)、八进制 (0o)、十六进制 (0x)
- **科学记数法**: 支持 1e3、2.5e-3 等格式，自动显示两种格式
- **输出格式转换**: 以十六进制、八进制或二进制显示结果
- **交互式 REPL**: 支持历史导航、Tab 补全和上次结果插入
- **数学常数**: 内置 `C_PI` 和 `C_E`，支持用户定义常量
- **比较运算符**: `<`, `>`, `=`, `==` 返回布尔值/yes-no 结果
- **完善的帮助系统**: `help [topic]` 获取详细文档

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
ralqlator "sin(C_PI / 2)"          # 输出：1

# 使用常数
ralqlator "C_PI * 2"               # 输出：6.28318
ralqlator "C_E ^ 2"                # 输出：7.389

# 比较运算符
ralqlator "5 > 3"                  # 输出：true
ralqlator "5 = 5"                  # 输出：yes

# 位运算
ralqlator -B "12 & 10"             # 输出：8
ralqlator -B "0xFF & 0x0F"         # 输出：15

# 输出格式转换
ralqlator -x "255"                 # 输出：0xFF
ralqlator -o "255"                 # 输出：0o377
ralqlator -b "255"                 # 输出：0b11111111

# 组合选项
ralqlator -Bx "0xFF & 0x0F"        # 输出：0xF

# 科学记数法（显示两种格式）
ralqlator "1e3 * 2"                # 输出：2000
                                 #   (scientific: 2.000000e3)

# 注意：负的非十进制数需要使用括号
ralqlator "-(0xFF)"                # 输出：-255
ralqlator "-0xFF"                  # 错误：不支持负的非十进制数
```

### 交互模式

```bash
ralqlator
```

交互模式下：
- 输入表达式并按 Enter 计算
- 使用 `@` 插入上次结果
- 使用 `hex`、`oct`、`bin` 将上次结果转换为不同格式
- 按 `Tab` 键进行命令补全（空行时显示帮助）
- 使用 `mode` 命令在标准模式和位运算模式之间切换
- 使用 `help [topic]` 获取特定主题的帮助
- 使用 `create func/seq/const` 定义函数、数列或常量
- 输入 `q` 或 `quit` 退出

```
> 255
255
> hex
0xFF
> @ + 1
256
> create const G 9.81
Constant 'G' = 9.81
> G * 10
98.1
> mode
切换到标准模式（浮点运算）
> 12 & 10
8
> help operators
运算符 - 支持的运算符
...
> quit
```

### 交互模式命令

| 命令 | 说明 |
|------|------|
| `Tab` | 命令补全（空行时显示帮助） |
| `@` | 插入上次计算结果 |
| `hex` / `oct` / `bin` | 将上次结果转换为对应格式 |
| `mode` | 切换计算模式（标准↔位运算） |
| `mode standard` | 切换到标准模式 |
| `mode bitwise` | 切换到位运算模式 |
| `help` | 显示完整帮助信息 |
| `help [topic]` | 显示特定主题帮助（functions, operators, formats, constants, mode, create, standard, bitwise） |
| `operators` | 显示当前模式运算符 |
| `functions` | 显示数学函数 |
| `formats` | 显示数字格式 |
| `constants` | 显示数学常数 |
| `create func` | 定义自定义函数：`create func f(x,y) = x+y` |
| `create seq` | 定义数列：`create seq a(n) = n*(n+1)/2` |
| `create const` | 定义常量：`create const NAME value` |
| `q` / `quit` | 退出 |

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
| `!` | 阶乘（后缀） | `5!` | 120 |

### 比较运算符

| 运算符 | 说明 | 示例 | 结果 |
|--------|------|------|------|
| `<` | 小于 | `3 < 5` | `true` |
| `>` | 大于 | `5 > 3` | `true` |
| `=` | 相等判断 | `5 = 5` | `yes` |
| `==` | 逻辑相等 | `5 == 5` | `true` |

**注意：**
- `<` 和 `>` 返回 `true` 或 `false`（关系比较）
- `=` 返回 `yes` 或 `no`（相等判断）
- `==` 返回 `true` 或 `false`（逻辑相等）

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

### 内置函数

| 函数 | 说明 | 示例 | 结果 |
|------|------|------|------|
| `lg(x)` | 以 10 为底对数 | `lg(100)` | 2 |
| `lg(x, base)` | 自定义底对数 | `lg(8, 2)` | 3 |
| `log(x, base)` | 自定义底对数 | `log(27, 3)` | 3 |
| `ln(x)` | 自然对数 | `ln(C_E)` | 1 |
| `sqrt(x)` | 平方根 | `sqrt(16)` | 4 |
| `pow(x, y)` | 幂函数 | `pow(2, 10)` | 1024 |
| `sin(x)` | 正弦（弧度） | `sin(0)` | 0 |
| `cos(x)` | 余弦（弧度） | `cos(0)` | 1 |
| `tan(x)` | 正切（弧度） | `tan(0)` | 0 |
| `asin(x)` | 反正弦 | `asin(1)` | π/2 |
| `acos(x)` | 反余弦 | `acos(1)` | 0 |
| `atan(x)` | 反正切 | `atan(1)` | π/4 |
| `mod(a, b)` | 取余函数 | `mod(10, 3)` | 1 |
| `factorial(n)` | 阶乘 (n!) | `factorial(5)` | 120 |
| `sum(a,b,...)` | 多参数求和 | `sum(1,2,3,4,5)` | 15 |
| `prod(a,b,...)` | 多参数连乘 | `prod(1,2,3,4,5)` | 120 |
| `suma(s,b,e)` | 数列求和 | `suma(triangle, 1, 5)` | 35 |

### 用户定义函数

在交互模式中创建自定义函数：
```
create func name(args) = expression
```

示例：
```
> create func f(x) = x^2
> f(5)
25

> create func add(a,b) = a+b
> add(3, 7)
10
```

### 用户定义数列

创建数列（单变量函数）供 `suma()` 使用：
```
create seq name(n) = formula
```

示例：
```
> create seq triangle(n) = n*(n+1)/2
> triangle(10)
55
> suma(triangle, 1, 5)
35

> create seq square(n) = n^2
> suma(square, 1, 5)
55
```

### 用户定义常量

在交互模式中创建自定义常量：
```
create const NAME value
```

示例：
```
> create const G 9.81
Constant 'G' = 9.81

> create const SPEED_OF_LIGHT 299792458
Constant 'SPEED_OF_LIGHT' = 299792458

> G * 10
98.1
```

注意：常量名不能以 `C_` 开头（保留给内置常量）。

## 常数

### 内置常数

| 常数 | 值 | 说明 |
|------|-----|------|
| `C_PI` | ≈ 3.14159 | 圆周率（周长与直径之比） |
| `C_E` | ≈ 2.71828 | 自然常数 e（自然对数底） |

注意：常量使用 `C_` 前缀格式。以 `C_` 开头的名称是保留的。

### 示例

```bash
> C_PI * 2
6.28318

> C_E ^ 2
7.389

> sin(C_PI / 2)
1
```

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

参数:
  [EXPR]  要计算的表达式

选项:
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

命令:
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

测试套件包含 **425+ 个测试用例**，分为 12 个测试文件：

### 测试组织

- **arithmetic_tests.rs** (45 个测试): 基本算术运算
- **bitwise_tests.rs** (38 个测试): 位运算
- **cli_tests.rs** (37 个测试): CLI 参数和选项
- **comparison_tests.rs** (22 个测试): 比较运算符
- **constants_tests.rs** (12 个测试): 数学常数
- **edge_cases_tests.rs** (50 个测试): 边界条件
- **error_handling_tests.rs** (41 个测试): 错误处理
- **functions_tests.rs** (66 个测试): 数学函数
- **interactive_tests.rs** (59 个测试): REPL 交互模式
- **internal_tests.rs** (12 个测试): 内部模块测试
- **number_formats_tests.rs** (34 个测试): 数字格式输入输出
- **user_defined_tests.rs** (23 个测试): 用户定义函数/常量

### 测试覆盖

- 基本算术运算
- 数字格式解析（二进制、八进制、十六进制）
- 数学常数（C_PI, C_E）
- 函数计算（内置和用户定义）
- 三角函数
- 位运算
- 阶乘、求和、连乘和数列求和函数
- 数列运算
- 用户定义函数、数列和常量
- 比较运算符（<, >, =, ==）
- 错误处理和非法输入
- 边界情况（嵌套括号、大/小数）
- 词法分析和语法分析
- 交互式 REPL 命令

## 项目结构

```
ralqlator/
├── Cargo.toml              # 项目配置
├── README.md               # 英文文档
├── README_zh.md            # 中文文档
├── agent.md                # AI 助手文档
└── src/
    ├── main.rs             # 程序入口
    ├── cli.rs              # CLI 参数定义
    ├── repl.rs             # 交互式 REPL（支持模式切换）
    ├── calculator.rs       # 计算编排
    ├── evaluator.rs        # 表达式求值
    ├── functions.rs        # 数学函数
    ├── operator.rs         # 运算符定义
    ├── shunting_yard.rs    # 中缀转后缀算法
    └── token.rs            # 词法分析

tests/
├── arithmetic_tests.rs     # 基本算术运算
├── bitwise_tests.rs        # 位运算
├── cli_tests.rs            # CLI 参数和选项
├── comparison_tests.rs     # 比较运算符
├── constants_tests.rs      # 数学常数
├── edge_cases_tests.rs     # 边界条件
├── error_handling_tests.rs # 错误处理
├── functions_tests.rs      # 数学函数
├── interactive_tests.rs    # REPL 交互模式
├── internal_tests.rs       # 内部模块测试
├── number_formats_tests.rs # 数字格式输入输出
└── user_defined_tests.rs   # 用户定义函数/常量
```

## 示例

```bash
# 复杂表达式
ralqlator "sqrt(pow(3, 2) + pow(4, 2))"    # 输出：5

# 科学记数法（显示两种格式）
ralqlator "1e3 + 2.5e-3"                   # 输出：1000.0025
                                 #   (scientific: 1.002500e3)

# 混合格式
ralqlator "0xFF + 0b1010"                  # 输出：265

# 使用常数
ralqlator "C_PI * 2"                       # 输出：6.28318
ralqlator "sin(C_PI / 2) + cos(0)"         # 输出：2

# 比较运算符
ralqlator "5 > 3"                          # 输出：true
ralqlator "5 = 5"                          # 输出：yes

# 对数
ralqlator "lg(1000)"                       # 输出：3
ralqlator "log(8, 2)"                      # 输出：3

# 位运算带格式输出
ralqlator -Bb "8 << 2"                     # 输出：0b100000

# 负的非十进制数（使用括号）
ralqlator "-(0xFF)"                        # 输出：-255

# 用户定义函数
ralqlator
> create func f(x) = x^2
> f(5)
25

# 用户定义数列
> create seq triangle(n) = n*(n+1)/2
> suma(triangle, 1, 5)
35

# 用户定义常量
> create const G 9.81
> G * 10
98.1

# 帮助系统
> help operators
> help functions
> help create
```

## 许可证

GNU General Public License v3.0 - 详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 致谢

- 使用 [Rust](https://www.rust-lang.org/) 构建
- CLI 解析使用 [clap](https://github.com/clap-rs/clap)
- 交互式输入使用 [rustyline](https://github.com/kkawakam/rustyline)
