---
description: |
   Typst 语法简略参考索引。更多请参考语言的标记模式，数学公式模式和代码模式。
---

# 语法

Typst 是一种标记语言。
这意味着，使用简单的语法就可以进行常用的布局操作，
再辅以 set 和 show 规则，格式化文档更加简单，更加自动化，
这些均是基于紧密集成在 Typst 内的脚本语言，
其内置大量常用函数，用户亦可根据需求自定义函数。

## 标记模式 { #markup }

Typst 为常用文档元素内置了语法标记。
这些语法标记大多只是相关函数的快捷表达方式，
下表列出了所有语法标记，以及它们的详细使用的链接地址。

| 名称            | 示例                      | 详情链接                     |
| --------------- | ------------------------ | ---------------------------- |
| 段落中断        | 空行                      | [`parbreak`]($parbreak) |
| 着重强调        | `[*strong*]`             | [`strong`]($strong)     |
| 强调            | `[_emphasis_]`           | [`emph`]($emph)         |
| 代码段          | ``[`print(1)`]``         | [`raw`]($raw)           |
| 链接            | `[https://typst.app/]`   | [`link`]($link)         |
| 标签            | `[<intro>]`              | [`label`]($label)       |
| 引用            | `[@intro]`               | [`ref`]($ref)           |
| 标题            | `[= Heading]`            | [`heading`]($heading)   |
| 无序列表        | `[- item]`               | [`list`]($list)         |
| 有序列表        | `[+ item]`               | [`enum`]($enum)         |
| 术语列表        | `[/ Term: description]`  | [`terms`]($terms)       |
| 数学公式        | `[$x^2$]`                | [Math]($category/math)       |
| 行中断          | `[\]`                    | [`linebreak`]($linebreak) |
| 智能引号        | `['single' or "double"]` | [`smartquote`]($smartquote) |
| 快捷符号        | `[~, ---]`               | [Symbols]($category/symbols/sym) |
| 代码表达式      | `[#rect(width: 1cm)]`    | [Scripting]($scripting/#expressions) |
| 转义字符        | `[Tweet at us \#ad]`     | [Below](#escapes)            |
| 注释            | `[/* block */, // line]` | [Below](#comments)           |

## 数学模式 { #math }

数学模式是一种特殊的语法标记模式，专门用来输入数学公式。
通过 `[$]` 字符包裹一个数学公式，
如果这个公式头尾都至少一个空格（例如`[$ x^2 $]`），这个公式将会形成一个文档块，单独占用一行，
如果头尾没有空格（例如`[$x^2$]`），这个公式将会排版在行内，
下面是针对数学模式的语法概述：

| 名称                  | 示例                    | 详情链接                      |
| --------------------- | ------------------------ | ------------------------ |
| 行内数学公式           | `[$x^2$]`                | [Math]($category/math)   |
| 块级数学公式           | `[$ x^2 $]`              | [Math]($category/math)   |
| 底部附缀              | `[$x_1$]`                | [`attach`]($category/math/attach) |
| 顶部附缀              | `[$x^2$]`                | [`attach`]($category/math/attach) |
| 分数                  | `[$1 + (a+b)/5$]`        | [`frac`]($math.frac) |
| 行中断                | `[$x \ y$]`              | [`linebreak`]($linebreak) |
| 对齐点                | `[$x &= 2 \ &= 3$]`      | [Math]($category/math)   |
| 变量访问              | `[$#x$, $pi$]`           | [Math]($category/math)   |
| 字段访问              | `[$arrow.r.long$]`       | [Scripting]($scripting/#fields) |
| 隐式乘积              | `[$x y$]`                | [Math]($category/math)   |
| 快捷符号              | `[$->, !=$]`             | [Symbols]($category/symbols/sym) |
| 数学公式内字符串       | `[$a "is natural"$]`     | [Math]($category/math)   |
| 数学函数调用          | `[$floor(x)$]`           | [Math]($category/math)   |
| 代码表达式            | `[$#rect(width: 1cm)$]`  | [Scripting]($scripting/#expressions) |
| 转义字符              | `[$x\^2$]`               | [Below](#escapes)        |
| 注释                 | `[$/* comment */$]`      | [Below](#comments)       |

## 代码模式 { #code }

在代码块和表达式中，新的表达式不再前缀 `#` 字符。
许多代码语法元素是表达式特有的，
下面列出了代码模式下所有可用的语法：


| 名称                     | 示例                          | 详情链接                              |
| ------------------------ | ----------------------------- | ---------------------------------- |
| 变量访问                 | `{x}`                         | [Scripting]($scripting/#blocks)    |
| 字面常量                 | `{1pt, "hey"}`                | [Scripting]($scripting/#expressions) |
| 代码块                   | `{{ let x = 1; x + 2 }}`      | [Scripting]($scripting/#blocks)    |
| 文档内容块               | `{[*Hello*]}`                 | [Scripting]($scripting/#blocks)    |
| 括号表达式               | `{(1 + 2)}`                   | [Scripting]($scripting/#blocks)    |
| 数组                    | `{(1, 2, 3)}`                 | [Array]($array)               |
| 字典                    | `{(a: "hi", b: 2)}`           | [Dictionary]($dictionary)     |
| 一元运算符              | `{-x}`                        | [Scripting]($scripting/#operators) |
| 二元运算符              | `{x + y}`                     | [Scripting]($scripting/#operators) |
| 赋值                   | `{x = 1}`                     | [Scripting]($scripting/#operators) |
| 字段访问               | `{x.y}`                       | [Scripting]($scripting/#fields)    |
| 方法调用               | `{x.flatten()}`               | [Scripting]($scripting/#methods)   |
| 函数调用               | `{min(x, y)}`                 | [Function]($function)         |
| 匿名函数               | `{(x, y) => x + y}`           | [Function]($function)         |
| let 绑定               | `{let x = 1}`                 | [Scripting]($scripting/#bindings)  |
| 命名函数               | `{let f(x) = 2 * x}`          | [Function]($function)         |
| set 规则               | `{set text(14pt)}`            | [Styling]($styling/#set-rules)     |
| set-if 规则            | `{set text(..) if .. }`       | [Styling]($styling/#set-rules)     |
| show-set 规则          | `{show par: set block(..)}`   | [Styling]($styling/#show-rules)    |
| 函数式 show 规则        | `{show raw: it => {..}}`      | [Styling]($styling/#show-rules)    |
| show-everything 规则   | `{show: columns.with(2)}`     | [Styling]($styling/#show-rules)    |
| 条件表语句             | `{if x == 1 {..} else {..}}`  | [Scripting]($scripting/#conditionals) |
| for 循环               | `{for x in (1, 2, 3) {..}}`   | [Scripting]($scripting/#loops)     |
| while 循环             | `{while x < 10 {..}}`         | [Scripting]($scripting/#loops)     |
| 循环流程控制            | `{break, continue}`           | [Scripting]($scripting/#loops)     |
| 函数返回                | `{return x}`                  | [Function]($function)         |
| include 模块           | `{include "bar.typ"}`         | [Scripting]($scripting/#modules)   |
| import 模块            | `{import "bar.typ"}`          | [Scripting]($scripting/#modules)   |
| 从模块内 import 条目   | `{import "bar.typ": a, b, c}` | [Scripting]($scripting/#modules)   |
| 注释                  | `[/* block */, // line]`      | [Below](#comments)                 |

## 注释 { #comments }

Typst 会忽略注释，最终生成的文档不会包含它们。
它们通常被用于剔除旧版本，或者添加标注说明。
如果一行开头是 `//`，这行就会被认为是注释：

```example
// our data barely supports
// this claim

We show with $p < 0.05$
that the difference is
significant.
```

也可以通过 `/*` 和 `*/` 来包裹注释，这种方式，注释可以分布于多行：

```example
Our study design is as follows:
/* Somebody write this up:
   - 1000 participants.
   - 2x2 data design. */
```

## 转义序列 { #escapes }

转义序列可以用来插入难于输入的特殊字符，或者 Typst 内有特殊含义的字符。
前缀一个反斜杠转义一个字符，转移序列如果是十六进制，
比如 `[\u{1f600}]`，就会插入一个 Unicode 码点。
这些类型的转义序列也作用于[字符串]($str)中。

```example
I got an ice cream for
\$1.50! \u{1f600}
```