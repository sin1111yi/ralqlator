# Ralqlator

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Build Status](https://img.shields.io/badge/test-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-GPLv3-blue)]()
[![Rust](https://img.shields.io/badge/rust-2021-orange)]()
[![Version](https://img.shields.io/badge/version-v0.3.1-blue)]()

> **AI 生成项目**: 本项目完全由 **qwen3.5-plus**（阿里巴巴通义千问大语言模型）在 **qwen-coder**（阿里巴巴 AI agent）下生成。

一个用 Rust 编写的强大命令行计算器，支持精确有理数运算、丰富的数学函数库和交互式 REPL。

## 功能特性

### 核心算术
- **基本算术**: 加法、减法、乘法、除法、取余、幂运算
- **精确有理数运算**: 精确分数计算（如 `1/3 + 1/3 + 1/3 = 1`）
- **位运算**: 与、或、异或、非、左移、右移（使用 `-B` 标志）

### 数学函数
- **对数函数**: `lg(x)`, `lg(x, base)`, `log(x, base)`, `ln(x)`, `log2(x)`
- **根式与幂**: `sqrt(x)`, `cbrt(x)`, `pow(x, y)`
- **三角函数**: `sin`, `cos`, `tan`, `sec`, `csc`, `cot` 及其反函数
- **双曲函数**: `sinh`, `cosh`, `tanh` 及其反函数
- **特殊函数**: `factorial`, `gamma`, `erf`, `erfc`, `beta`
- **BigInt 函数**: `bfactorial`, `bpow`, `comb`, `perm`, `gcd`, `lcm`, `isprime`, `nextprime`

### 有理数函数（v0.3.0 新增）
- **`num(x)`**: 获取分子
- **`den(x)`**: 获取分母
- **`frac(x)`**: 获取小数部分
- **`rational(n, d)`**: 从分子/分母创建有理数
- **`float(x)`**: 转换为浮点数
- **分数输入**: 直接输入 `1/3`, `22/7` 等分数

### 用户定义元素
- **函数**: `create func name(args) = expression`
- **数列**: `create seq name(n) = formula`
- **常量**: `create const NAME value`

### 数字格式
- **输入**: 十进制、二进制 (`0b`)、八进制 (`0o`)、十六进制 (`0x`)、科学记数法
- **输出**: 十六进制 (`-x`)、八进制 (`-o`)、二进制 (`-b`)

### 交互式 REPL
- 箭头键历史导航
- Tab 命令补全
- 上次结果插入 (`@`)
- 模式切换 (`mode` 命令)
- 完善的帮助系统 (`help [topic]`)

### 比较运算符
- `<`, `>`, `=`, `==` 统一返回 `true` 或 `false`

### 帮助系统
- CLI: `ralqlator --help`, `ralqlator functions`, `ralqlator operators` 等
- REPL: `help`, `help functions`, `help operators`, `help create` 等
- 版本信息：`ralqlator -v` 或 `ralqlator --version`

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
- `=` 返回 `true` 或 `no`（相等判断）
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

测试套件包含 **427+ 个测试用例**，分为 9 个测试文件：

### 测试组织

- **01_core_tests.rs** (75 个测试): 核心算术、位运算、比较运算、紧凑格式测试
- **02_functions_tests.rs** (35 个测试): 数学函数
- **03_cli_formats_tests.rs** (39 个测试): CLI 参数、数字格式、科学记数法
- **04_repl_tests.rs** (55 个测试): REPL 交互模式
- **05_rational_tests.rs** (49 个测试): 有理数运算和解析器
- **06_error_internal_tests.rs** (46 个测试): 错误处理和内部模块
- **07_user_defined_tests.rs** (8 个测试): 用户定义常量
- **08_extended_tests.rs** (86 个测试): 扩展功能和边界情况
- **e2e_integration_tests.rs** (34 个测试): 端到端集成测试

### 额外测试

- **基准测试** (`benches/calculator_bench.rs`): 24 个性能基准测试
- **模糊测试** (`fuzz/`): 解析器安全模糊测试

### 测试覆盖

- 基本和高级算术运算
- **紧凑格式输入**（无空格）：`1+1`, `2*3`, `1e+3`
- 精确有理数运算
- **紧凑分数语法**：`1/2+1/3`, `(1/2)*6`
- 数字格式解析（二进制、八进制、十六进制）
- 数学常数（C_PI, C_E）
- 所有数学函数（内置和用户定义）

## 项目结构

```
ralqlator/
├── Cargo.toml              # 项目配置
├── build.rs                # 从 git 生成版本信息
├── README.md               # 英文文档
├── README_zh.md            # 中文文档
├── TESTING.md              # 测试指南
├── HELP_UPDATE.md          # 帮助系统文档
├── OPTIMIZATION_SUMMARY.md # 优化总结
├── COMMIT_MESSAGE.md       # 提交信息模板
├── benches/                # 性能基准测试
│   └── calculator_bench.rs
├── fuzz/                   # 模糊测试配置
│   ├── Cargo.toml
│   └── fuzz_targets/
│       └── parse_expression.rs
├── scripts/                # 工具脚本
│   └── generate_version.sh
└── src/
    ├── main.rs             # 程序入口
    ├── cli.rs              # CLI 参数定义
    ├── repl.rs             # 交互式 REPL
    ├── calculator.rs       # 计算编排
    ├── evaluator.rs        # 表达式求值
    ├── functions.rs        # 数学函数
    ├── operator.rs         # 运算符定义
    ├── shunting_yard.rs    # 中缀转后缀算法
    ├── token.rs            # 词法分析
    ├── parser.rs           # AST 解析器（递归下降）[新增]
    ├── value.rs            # 统一 Value 类型 [新增]
    ├── rational.rs         # 有理数工具 [新增]
    ├── error.rs            # 错误处理 [新增]
    └── lib.rs              # 库导出 [新增]

tests/
├── 01_core_tests.rs        # 核心算术、位运算、比较
├── 02_functions_tests.rs   # 数学函数
├── 03_cli_formats_tests.rs # CLI 和数字格式
├── 04_repl_tests.rs        # REPL 交互模式
├── 05_rational_tests.rs    # 有理数运算
├── 06_error_internal_tests.rs # 错误处理和内部模块
├── 07_user_defined_tests.rs # 用户定义常量
├── 08_extended_tests.rs    # 扩展功能
├── e2e_integration_tests.rs # 端到端测试
└── README.md               # 测试文档
```

## 示例

### 标准格式（带空格）
```bash
ralqlator "1 + 2 * 3"                    # 输出：7
ralqlator "10 / 2 + 5"                   # 输出：10
```

### 紧凑格式（无空格）
```bash
ralqlator "1+2*3"                        # 输出：7
ralqlator "10/2+5"                       # 输出：10
ralqlator "1+1"                          # 输出：2
ralqlator "2^10"                         # 输出：1024
```

### 科学记数法
```bash
ralqlator "1e+3"                         # 输出：1000
ralqlator "1e-3"                         # 输出：0.001
ralqlator "1e+3+1e-3"                    # 输出：1000.001
ralqlator "1.5e+2"                       # 输出：150
```

### 分数输入
```bash
ralqlator "1/2"                          # 输出：0.5
ralqlator "1/2+1/3"                      # 输出：0.833333...
ralqlator "(1/2)*6"                      # 输出：3
```

### 复杂表达式
```bash
ralqlator "sqrt(pow(3, 2) + pow(4, 2))"  # 输出：5
ralqlator "2*3+4*5"                      # 输出：26
ralqlator "(1+2)*(3+4)"                  # 输出：21
ralqlator "10--5"                        # 输出：15
```

### 混合格式
```bash
ralqlator "0xFF + 0b1010"                # 输出：265
```

### 使用常数
```bash
ralqlator "C_PI * 2"                     # 输出：6.28318
ralqlator "sin(C_PI / 2) + cos(0)"       # 输出：2
```

### 比较运算符
```bash
ralqlator "5 > 3"                        # 输出：true
ralqlator "5 = 5"                        # 输出：true
ralqlator "5>3"                          # 输出：true  （紧凑格式）
```

### 对数
```bash
ralqlator "lg(1000)"                     # 输出：3
ralqlator "log(8, 2)"                    # 输出：3
```

### 位运算带格式输出
```bash
ralqlator -Bb "8 << 2"                   # 输出：0b100000
```

### 负的非十进制数
```bash
ralqlator "-(0xFF)"                      # 输出：-255
```

### 交互模式
```bash
ralqlator
> create func f(x) = x^2
> f(5)
25

> create seq triangle(n) = n*(n+1)/2
> suma(triangle, 1, 5)
35

> create const G 9.81
> G * 10
98.1

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
