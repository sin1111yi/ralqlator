# Ralqlator 测试文档

> **其他语言版本**: [English](README_en.md)

## 测试文件结构（9 个文件，397 个测试）

| 序号 | 文件名 | 测试数 | 说明 |
|------|--------|--------|------|
| 1 | `01_core_tests.rs` | 66 | 核心功能测试（算术、位运算、比较、常量、边界） |
| 2 | `02_functions_tests.rs` | 35 | 数学函数测试（对数、三角、BigInt 等） |
| 3 | `03_cli_formats_tests.rs` | 33 | CLI 参数和数字格式测试 |
| 4 | `04_repl_tests.rs` | 55 | REPL 交互模式测试 |
| 5 | `05_rational_tests.rs` | 44 | 有理数和解析器测试 |
| 6 | `06_error_internal_tests.rs` | 46 | 错误处理和内部模块测试 |
| 7 | `07_user_defined_tests.rs` | 8 | 用户定义常量测试 |
| 8 | `08_extended_tests.rs` | 76 | 扩展综合测试 |
| 9 | `e2e_integration_tests.rs` | 34 | 端到端集成测试 |
| **总计** | **9 文件** | **397** | 全面覆盖 |

---

## 详细分类

### 1. 核心功能测试 (`01_core_tests.rs` - 66 tests)

#### 算术运算
- 加法：基本、大数、负数、多次加法
- 减法：基本、负结果、负数减法
- 乘法：基本、乘零、负数乘法
- 除法：基本、小数结果、除以 1、除零错误
- 取余：基本、取余零错误
- 幂运算：基本、零次幂、一次幂
- 阶乘：基本、0!、1!、大数阶乘

#### 位运算
- AND、OR、XOR、NOT
- 左移、右移、移位零
- 组合运算、十六进制输入

#### 比较运算
- 小于、大于、等于、双等于
- 真假值测试、表达式比较

#### 常量
- C_PI、C_E
- 常量表达式、常量与函数组合

#### 边界情况
- 零运算、负数、小数、大数
- 链式运算、混合运算、恒等运算

### 2. 函数测试 (`02_functions_tests.rs` - 35 tests)

#### 对数函数
- lg (以 10 为底)、自定义底数
- ln (自然对数)、log2

#### 根函数
- sqrt (平方根)、cbrt (立方根)

#### 幂函数
- pow

#### 三角函数
- sin, cos, tan
- asin, acos, atan

#### 双曲函数
- sinh, cosh, tanh

#### 实用函数
- abs, floor, ceil, round, mod
- sum, prod (多参数)

#### BigInt 函数
- bfactorial, bpow
- comb, perm
- gcd, lcm
- isprime, nextprime

#### 错误测试
- 负数平方根、负数对数
- 反正弦超出范围

### 3. CLI 和格式测试 (`03_cli_formats_tests.rs` - 33 tests)

#### 命令行参数
- 位置参数、-r 简写

#### 输出格式
- -x 十六进制、-o 八进制、-b 二进制
- 格式输出计算

#### 位运算模式
- -B 标志、组合标志

#### 输入格式
- 0b 二进制、0o 八进制、0x 十六进制
- 负前缀数字、混合格式

#### 科学记数法
- 1e3、1e-3、科学记数法运算

#### 帮助系统
- --help、-F、-O、-N、-C
- 子命令：functions, operators, formats, constants

#### 错误处理
- 非整数进制输出错误、无效表达式

### 4. REPL 测试 (`04_repl_tests.rs` - 55 tests)

#### 基本交互
- 算术运算、位运算模式
- 函数调用、常量使用

#### 用户定义
- 函数定义和使用
- 数列定义和 suma 求和
- 常量定义

#### 历史结果
- @ 引用、链式运算
- hex/oct/bin 转换

#### 比较运算
- <, >, =, ==

#### 数字格式
- 十六进制、二进制、八进制输入

#### 帮助命令
- help、help functions、help operators
- help constants

#### 错误处理
- 除零、无效表达式、未定义函数
- 括号不匹配

#### 边界情况
- 空行、空白、负数、嵌套括号

#### BigInt 函数
- bfactorial, bpow, gcd, lcm
- isprime, nextprime

### 5. 有理数测试 (`05_rational_tests.rs` - 33 tests)

#### 解析器测试
- 数字解析（十进制、二进制、八进制、十六进制、科学记数法）
- 标识符、函数调用、AST 节点构造
- 表达式解析（简单、括号）

#### 有理数 CLI
- 分数输入、有理数运算
- 有理数函数 CLI

#### 有理数 REPL
- 分数输入交互、有理数运算交互
- 用户定义函数/常量与有理数

#### 解析器集成
- AST 求值、括号、一元运算符
- 指数运算、比较运算
- 分数输入、嵌套函数
- 错误处理（除零、未定义函数）
- 右结合幂运算、混合运算

#### 有理数函数
- num, den, frac
- rational, float

### 6. 错误和内部测试 (`06_error_internal_tests.rs` - 46 tests)

#### 错误类型
- CalcError 显示、转字符串、字符串转错误

#### 除零错误
- 除法、取余、嵌套表达式

#### 定义域错误
- 负数平方根、负数对数
- asin/acosh/atanh 超出范围

#### 无效输入
- 空表达式、空白、仅运算符
- 无效数字

#### 语法错误
- 连续运算符、括号不匹配
- 空括号

#### 函数参数错误
- 无参数、过多参数、错误参数数
- 无效函数

#### 位运算错误
- 减法不支持、无效移位

#### 阶乘错误
- 负数、过大、非整数

#### Value 类型测试
- from_int, from_float
- add, div, pow
- 除零错误

#### 词法分析测试
- 简单表达式、前缀数字
- 负前缀数字

### 7. 用户定义测试 (`07_user_defined_tests.rs` - 8 tests)

#### 内置常量
- C_PI、C_E
- 常量表达式、常量与函数组合
- 常量与比较运算

### 8. 扩展测试 (`08_extended_tests.rs` - 76 tests)

#### 高级算术
- 复杂表达式、幂的幂、右结合幂
- 阶乘表达式、取余负数

#### 高级函数
- 嵌套函数、三角恒等式
- 对数性质、sum/prod 函数
- abs/floor/ceil/round

#### BigInt 扩展
- bfactorial(100)、bpow(2, 200)
- comb(52,5) 扑克牌组合
- perm、gcd 互质、lcm 互质
- isprime 质数/合数、nextprime

#### 数字格式扩展
- 二进制/八进制/十六进制运算
- 混合格式、格式输出计算

#### 科学记数法扩展
- 大数、小数、乘法、除法
- 负指数

#### REPL 扩展
- 模式切换、历史结果链
- hex/oct/bin 转换、空行帮助
- Tab 补全、创建和使用函数/常量
- quit 命令

#### 错误处理扩展
- 括号不匹配、空括号
- 连续运算符、仅运算符
- 无效函数、错误参数数
- sqrt/log/asin 错误
- factorial 错误、位运算错误

#### 帮助系统扩展
- 帮助标志、函数/运算符/格式/常量帮助
- 子命令、info 命令

#### 边界情况扩展
- 超大乘法、零的幂
- 0!/1!、小数精度
- 多小数、空白处理
- 单个数字、负数
- 浮点数比较

---

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定类别测试
cargo test --test 01_core_tests
cargo test --test 02_functions_tests
cargo test --test 03_cli_formats_tests
cargo test --test 04_repl_tests
cargo test --test 05_rational_tests
cargo test --test 06_error_internal_tests
cargo test --test 07_user_defined_tests
cargo test --test 08_extended_tests

# 运行单个测试
cargo test test_addition
cargo test test_sin
cargo test test_repl_addition

# 运行特定模式测试
cargo test cli           # CLI 相关测试
cargo test repl          # REPL 相关测试
cargo test rational      # 有理数相关测试
cargo test error         # 错误相关测试
```

---

## 测试覆盖范围

| 功能模块 | 测试文件 | 测试数 | 覆盖率 |
|----------|----------|--------|--------|
| 算术运算 | 01, 08 | ~80 | ✅ |
| 位运算 | 01, 08 | ~20 | ✅ |
| 比较运算 | 01, 04, 08 | ~25 | ✅ |
| 数学函数 | 02, 08 | ~50 | ✅ |
| BigInt 函数 | 02, 04, 08 | ~25 | ✅ |
| CLI 参数 | 03, 08 | ~40 | ✅ |
| 数字格式 | 03, 08 | ~25 | ✅ |
| REPL 交互 | 04, 08 | ~80 | ✅ |
| 有理数 | 05 | ~33 | ✅ |
| 解析器 | 05 | ~20 | ✅ |
| 错误处理 | 06, 08 | ~50 | ✅ |
| 内部模块 | 06 | ~15 | ✅ |
| 常量 | 01, 07 | ~15 | ✅ |
| 帮助系统 | 03, 08 | ~15 | ✅ |
| **总计** | **8 文件** | **~373** | **✅** |

---

## 优化说明

**测试文件组织：**
1. 按功能域分组（核心、函数、CLI、REPL、有理数、错误、用户定义、扩展）
2. 使用数字前缀保持文件顺序
3. 每个文件包含相关的子类别测试
4. 扩展测试文件包含跨类别的综合测试

**测试覆盖策略：**
- 基本功能测试
- 边界条件测试
- 错误处理测试
- 组合功能测试
- CLI 和 REPL 双模式测试
