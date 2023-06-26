---
description: |
  您是 LaTeX 用户吗？本指南解释了 Typst 和 LaTeX 之间的差异和相似之处，以便您可以快速入门。
---

# LaTeX 用户指南
如果你以前使用过 LaTeX 并想尝试 Typst，那么此页面是一个很好的开始。
我们将从用户的角度探讨这两个排版系统的主要区别。
尽管 Typst 不是基于 LaTeX 构建的，并且具有不同的语法，但是你可以利用你的 LaTeX 技能来快速入门。

<!-- Mention that Typst is not built upon LaTeX -->

就像 LaTeX 一样，Typst 是一个基于标记的排版系统：你在一个文本文件中编写文档，并用命令和其他语法对其进行标记。
然后，使用编译器将源文件排版为PDF。
然而，Typst 在几个方面不同于 LaTeX:
首先，Typst 对常见任务使用更专用的语法（就像你可能从 Markdown 中知道的那样）。
Typst 的命令也更有原则:它们都是一样的，所以不像在LaTeX中，你只需要理解一些通用的概念，而不是学习每个包的不同约定。
此外，Typst 的编译速度比 LaTeX 更快：编译通常需要几毫秒，而不是几秒钟，因此[Web应用程序](https://typst.app/)和编译器都可以提供实时预览。

下面，我们将介绍一些从 LaTeX 转换到 Typst 的用户在编写文档时会遇到的最常见的问题。
如果你更喜欢一步一步地介绍 Typst，请查看我们的[教程]($tutorial)。

## 如何创建一个新的空文档？{ #getting-started }
这很简单。您只需创建一个新的空文本文件（文件扩展名为.typ）。
开始时不需要任何模板。只需直接开始编写你的文字。文字将被渲染在一个空白的A4大小的页面上。
如果您使用的是Web应用程序，请单击“+ Empty document”以创建一个带有文件的新项目，然后进入编辑器。
[段落分隔符]($func/parbreak)的工作方式和 LaTeX 中的一样，只是使用一个空行。

```example
Hey there!

Here are two paragraphs. The
output is shown to the right.
```

## 我如何创建章节标题，强调，...？{ #elements }
LaTeX 使用`\section`命令创建章节标题。嵌套标题用`\subsection`、`\subsection`等表示。根据文档类型，还有`\part`或`\chapter`。

在 Typst 中，标题设置更简洁：在标题所在的行前面加上一个等号和一个空格，得到一级标题：`[= Introduction]`。
如果你需要一个二级标题，你可以使用两个等号：`[== In this paper]`。
你可以通过添加更多的等号，将标题嵌套到你想要的深度。

强调（通常以斜体字呈现）是通过用`[_underscores_]`来表达，
而着重的强调（通常以黑体字呈现）是通过使用`[*Star*]`来代替。

下面是 LaTeX 中使用的常见标记命令及其 Typst 对应的表示方式。你也可以查看[完整的语法备忘单]($syntax)。

| 元素              | LaTeX                     | Typst                  | See                      |
|:-----------------|:--------------------------|:-----------------------|:-------------------------|
| 着重强调          | `\textbf{strong}`         | `[*strong*]`           | [`strong`]($func/strong) |
| 强调             | `\emph{emphasis}`         | `[_emphasis_]`         | [`emph`]($func/emph)     |
| 等宽文字 / 代码    | `\texttt{print(1)}`       | ``[`print(1)`]``       | [`raw`]($func/raw)       |
| 链接             | `\url{https://typst.app}` | `[https://typst.app/]` | [`link`]($func/link)     |
| 标签             | `\label{intro}`           | `[<intro>]`            | [`label`]($func/label)   |
| 交叉引用          | `\ref{intro}`             | `[@intro]`             | [`ref`]($func/ref)       |
| 文献引用          | `\cite{humphrey97}`       | `[@humphrey97]`        | [`cite`]($func/cite)     |
| 无序列表          | `itemize` 环境            | `[- List]`             | [`list`]($func/list)     |
| 有序列表          | `enumerate` 环境          | `[+ List]`             | [`enum`]($func/enum)     |
| 描述列表          | `description` 环境        | `[/ Term: List]`       | [`terms`]($func/terms)   |
| 图片             | `figure` 环境             | `figure` 函数           | [`figure`]($func/figure) |
| 表格             | `table` 环境              | `table` 函数            | [`table`]($func/table)   |
| 公式             | `$x$`, `align` / `equation` 环境 | `[$x$]`, `[$ x = y $]` | [`equation`]($func/math.equation) |

[列表]($func/list)不依赖于 Typst 中的环境。相反，它们有像标题一样的轻量级语法。
要创建一个无序列表（`itemize`），只需要在每一行的项目前加一个`-`字符：

````example
To write this list in Typst...

```latex
\begin{itemize}
  \item Fast
  \item Flexible
  \item Intuitive
\end{itemize}
```

...just type this:

- Fast
- Flexible
- Intuitive

````

嵌套列表通过使用适当的缩进来实现。在项目之间添加一个空行会使列表的间距更大。

要获得一个[有序列表]($func/enum)（`enumerate`），请用`+`代替连字符。
对于一个[描述列表]($func/terms)（`description`），用`[/ Term: Description]`来代替。

## 我如何使用一个命令？ { #commands }
LaTeX 在很大程度上依赖于命令（以反斜线为前缀）。它使用 _宏_ 来影响排版过程，并插入和操作内容。
有些命令接受参数，这些参数经常被放在大括号里： `\cite{rasmus}`。

Typst 区分了[标记模式和代码模式]($scripting/#blocks)。默认情况下是标记模式，在这种模式下，你可以编排文本并应用语法结构，如 `[*星代表粗体文本*]`。
另一方面，代码模式与Python等编程语言相似，提供输入和执行代码段的选项。

在 Typst 的标记中，你可通过使用一个标签（`#`）为一个单一的命令（或者说，_表达式_）切换到代码模式。
这是你调用函数的方式，例如，将你的项目分割成不同的[文件]($scripting/#modules)或根据某些[条件]($scripting/#conditionals)渲染文本。
在代码模式下，可以通过使用方括号来包含正常的标记[内容]($type/content)。在代码模式下，这些内容就像变量的任何其他正常值一样被处理。

```example
First, a rectangle:
#rect()

Let me show how to do
#underline([_underlined_ text])

We can also do some maths:
#calc.max(3, 2 * 4)

And finally a little loop:
#for x in range(3) [
  Hi #x.
]
```

一个函数调用总是涉及到函数的名称（[`rect`]($func/rect), [`underline`]($func/underline), 
[`calc.max`]($func/calc.max), [`range`]($func/range)），后面是小括号（与 LaTeX 不同，如果宏不需要参数，方括号和大括号是可选的）。
在这些圆括号内传递的参数的预期列表取决于具体的函数，并在[引用]($reference)中指定。

### 参数 { #arguments }
一个函数可以有多个参数。有些参数是条件性的，也就是说，你只需提供值： 函数`[#lower("SCREAM")]`以全小写的方式返回其参数。
许多函数使用命名参数而不是位置参数，以增加可读性。例如，一个矩形的尺寸和笔划是用命名参数定义的：

```example
#rect(
  width: 2cm,
  height: 1cm,
  stroke: red,
)
```

你指定一个命名参数，首先输入它的名字（上文中是它`width`、`height`和`stroke`），然后是冒号，接着是值（`2cm`、`1cm`、`red`）。
你可以在每个函数的参考页中找到可用的命名参数，或者在输入时的自动完成面板中找到。
命名参数类似于一些 LaTeX 环境的配置方式，例如，你可以输入`\begin{enumerate}[label={alph*)}]`来启动一个带有标签`a)`、`b)`等的列表。

通常，你想为一个函数提供一些[内容]($type/content)。
例如，LaTeX 的命令 `\underline{Alternative A}` 在 Typst 中会翻译成 `#underline([Alternative A])`。
方括号表示一个值是内容。在这些方括号内，你可以使用正常的标记。
然而，对于一个相当简单的结构来说，这是个很大的括号。
这就是为什么你也可以把后面的内容参数移到小括号之后（如果小括号最后是空的，就省略掉）。

```example
Typst is an #underline[alternative]
to LaTeX.

#rect(fill: aqua)[Get started here!]
```

### 数据类型 { #data-types }
你可能已经注意到，这些参数有独特的数据类型。Typst 支持许多[数据类型]($type)。
下面有一个表格，列出了一些最重要的数据类型以及如何编写它们。为了指定这些类型的值，你必须在代码模式下才行!

| Data type                            | Example                           |
|:-------------------------------------|:----------------------------------|
| [Content]($type/content)             | `{[*fast* typesetting]}`          |
| [String]($type/string)               | `{"Pietro S. Author"}`            |
| [Integer]($type/integer)             | `{23}`                            |
| [Floating point number]($type/float) | `{1.459}`                         |
| [Absolute length]($type/length)      | `{12pt}`, `{5in}`, `{0.3cm}`, ... |
| [Relative length]($type/ratio)       | `{65%}`                           |

内容和字符串的区别在于，内容可以包含标记，包括函数调用，而字符串实际上只是一个普通的字符序列。

Typst 提供了[控制流结构]($scripting/#conditionals)和[运算符]$(scripting#operators)，
如用于添加东西的 `+` 或用于检查两个变量之间是否相等的 `==`。
你也可以定义你自己的[变量]($scripting/#bindings)并对其进行计算。

### 影响整个文档的命令 { #rules }
在 LaTeX 中，有些命令，如 `\textbf{bold text}`，在大括号中接收一个参数，只影响该参数。
其他命令，如 `\bfseries bold text` 作为开关，改变文档或当前范围内所有后续内容的外观。

在 Typst 中，同一个函数既可以用来影响文档的其余部分、一个块（或范围）的外观，也可以只影响其参数。
例如，`[#text(weight: "bold")[bold text]]` 将只对其参数加粗，
而 `[#set text(weight: "bold")]` 将对当前块或直到文件结尾之前的所有文本加粗。
一个函数的效果根据它在调用或[设置规则]($styling/#set-rules)中的使用而立即显现。

```example
I am starting out with small text.

#set text(14pt)

This is a bit #text(18pt)[larger,]
don't you think?
```

设置规则可以出现在文件的任何地方。它们可以被认为是其各自函数的默认参数值：

```example
#set enum(numbering: "I.")

Good results can only be obtained by
+ following best practices
+ being aware of current results
  of other researchers
+ checking the data for biases
```

`+` 是调用[`{enum}`]($func/enum)函数的语法糖（可以把它看作是缩写），我们在上面应用了一个集合规则。
[大多数语法都是以这种方式与一个函数相连的]($syntax)。
如果你需要对一个元素进行超出其参数所能实现的样式设置，你可以用一个[显示规则]($styling/#show-rules)
（有点类似于 `\renewcommand`）来完全重新定义其外观。

## 如何加载一个文档类？ { #templates }
在 LaTeX 中，你用 `\documentclass{article}` 命令开始你的主 `.tex` 文件，以定义你的文件应该是什么样子。
在该命令中，你可能用另一个值代替 `article` ，如 `report` 和 `amsart` 来选择不同的外观。

当使用 Typst 时，你用[函数]($type/function)来为你的文档样式。
通常情况下，你使用一个模板，它提供了一个函数来为你的整个文档样式。首先，你从模板文件中导入该函数。
然后，你把它应用到你的整个文档中。这是用一个[显示规则]($styling/#show-rules)来完成的，该规则用一个给定的函数来包装下面的文档。
下面的例子说明了它是如何工作的：

```example:single
>>> #let conf(
>>>   title: none,
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
>>>  set text(font: "Linux Libertine", 11pt)
>>>  set par(justify: true)
>>>  set page(
>>>    "us-letter",
>>>    margin: auto,
>>>    header: align(
>>>      right + horizon,
>>>      title
>>>    ),
>>>    numbering: "1",
>>>  )
>>>
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )
>>>
>>>  set align(center)
>>>  text(17pt, title)
>>>
>>>  let count = calc.min(authors.len(), 3)
>>>  grid(
>>>    columns: (1fr,) * count,
>>>    row-gutter: 24pt,
>>>    ..authors.map(author => [
>>>      #author.name \
>>>      #author.affiliation \
>>>      #link("mailto:" + author.email)
>>>    ]),
>>>  )
>>>
>>>  par(justify: false)[
>>>    *Abstract* \
>>>    #abstract
>>>  ]
>>>
>>>  set align(left)
>>>  columns(2, doc)
>>>}
<<< #import "conf.typ": conf
#show: conf.with(
  title: [
    Towards Improved Modelling
  ],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
)

Let's get started writing this
article by putting insightful
paragraphs right here!
```

[`import`]($scripting/#modules) 语句使另一个文件中的函数（和其他定义）可用。
在这个例子中，它从 `conf.typ` 文件中导入 `conf` 函数。这个函数将一个文档格式化为一篇会议文章。
我们使用一个显示规则将其应用到文档中，同时配置文章的一些元数据。应用显示规则后，我们就可以马上开始写文章了!

<div class="info-box">

在 Typst 中，函数被称为"命令"，它们可以将其参数转化为输出值，包括文档 _内容_。
函数是"纯净"的，这意味着它们除了创建一个输出值/输出内容外，不能产生任何其他效果。
这与 LaTeX 的宏形成了鲜明的对比，后者可以对你的文档产生任意的效果。

要让一个函数为你的整个文档提供样式，显示规则会处理它后面的所有内容，并以结果为参数调用冒号后面指定的函数。
`.with` 部分是一个方法，它接收 `conf` 函数，并在传递给显示规则之前预先配置它的一些参数。
</div>

在Web应用程序中，你可以从预定义的模板中选择，甚至可以使用模板向导创建自己的模板。
你也可以查看 [`awesome-typst` 仓库](https://github.com/qjcg/awesome-typst)，找到由社区制作的模板。
我们计划建立一个包管理器，使模板在未来更容易被分享!

你也可以[创建你自己的、自定义的模板]($tutorial/making-a-template)。它们比相应的 LaTeX 的 `.sty` 文件短得多，可读性也高得多，所以不妨一试!

## 如何导入包？ { #packages }
Typst 是"即插即用"的，所以许多流行的 LaTeX 包的对应表达是直接内置的。
下面，我们编制了一个表格，其中包括经常使用的 LaTeX 包和它们相应的 Typst 函数。

| LaTeX 包                   | Typst 替代                                                    |
|:--------------------------------|:---------------------------------------------------------------------|
| graphicx, svg                   | [`image`]($func/image) function                                      |
| tabularx                        | [`table`]($func/table), [`grid`]($func/grid) functions               |
| fontenc, inputenc, unicode-math | Just start writing!                                                  |
| babel, polyglossia              | [`text`]($func/text.lang) function: `[#set text(lang: "zh")]`        |
| amsmath                         | [Math mode]($category/math)                                          |
| amsfonts, amssymb               | [`sym`]($category/symbols) module and [syntax]($syntax/#math)        |
| geometry, fancyhdr              | [`page`]($func/page) function                                        |
| xcolor                          | [`text`]($func/text.fill) function: `[#set text(fill: rgb("#0178A4"))]` |
| hyperref                        | [`link`]($func/link) function                                        |
| bibtex, biblatex, natbib        | [`cite`]($func/cite), [`bibliography`]($func/bibliography) functions |
| lstlisting, minted              | [`raw`]($func/raw) function and syntax                               |
| parskip                         | [`block`]($func/block.spacing) and [`par`]($func/par.first-line-indent) functions |
| csquotes                        | Set the [`text`]($func/text.lang) language and type `["]` or `[']`   |
| caption                         | [`figure`]($func/figure) function                                    |
| enumitem                        | [`list`]($func/list), [`enum`]($func/enum), [`terms`]($func/terms) functions |

如果你需要从另一个文件中加载函数和变量，例如，为了使用一个模板，你可以使用 [`import`]($scripting/modules) 语句。
如果你想包括另一个文件的文本内容，你可以使用 [`include`]($scripting/#modules) 语句。它将检索指定文件的内容并将其放入你的文档中。

目前，Typst 没有软件包管理器，但我们计划建立一个，这样你就可以轻松地使用社区中的工具和模板的软件包，并发布你自己的。
在那之前，你可能想看看 [awesome-typst 仓库](https://github.com/qjcg/awesome-typst)，它包含了一个为 Typst 创建的库的精选列表。

## 如何输入数学公式？ { #maths }
要在 Typst 中进入数学模式，只需将方程用`$`符号括起来。你可以通过在方程内容和其周围的`$`符号之间添加空格或换行来进入显示模式。

```example
The sum of the numbers from
$1$ to $n$ is:

$ sum_(k=1)^n k = (n(n+1))/2 $
```

[数学模式]($category/math)的工作方式与普通标记或代码模式不同。数字和单个字符被逐字显示，而多个连续（非数字）字符将被解释为Typst变量。

Typst在数学模式下预先定义了很多有用的变量。所有希腊字母（`alpha`, `beta`, ...）和一些希伯来字母（`alef`, `bet`, ...）都可以通过它们的名字使用。
一些符号还可以通过速记法使用，如`<=`、`>=`和`->`。

符号的完整列表请参考[符号页面]($func/symbol)。如果缺少一个符号，你也可以通过[Unicode转义序列]($syntax/#escapes)访问它。

符号的替代和相关形式通常可以通过在句点后附加一个[修饰符]($type/symbol)来选择。例如，`arrow.l.squiggly`插入了一个向左倾斜的箭头。
如果你想在你的表达式中插入多字母文本，请用双引号将其括起来：

```example
$ delta "if" x <= 5 $
```

在Typst中，定界符将为其表达式自动缩放，就像LaTeX中的 `\left` 和 `\right` 命令是隐式插入的。
你可以使用[`lr`]($func/math.lr)函数自定义定界符的行为。为了防止一对定界符的缩放，你可以用反斜线转义。

Typst会自动将斜线 `/` 周围的术语设置为分数，同时尊重运算符的优先级。所有的圆括号都会出现在输出中，而不会因为分数而变得多余。

```example
$ f(x) = (x + 1) / x $
```

[下标和上标]($func/math.attach)在 Typst 和 LaTeX 中的作用是相似的。`{$x^2$}`将产生一个上标，`{$x_2$}`产生一个下标。
如果你想在下标或上标中包含一个以上的值，请把它们的内容放在括号里：`{$x_(a -> epsilon)$}`。

由于数学模式下的变量不需要在前面加上 `#` 或 `/` ，所以你也可以调用没有这些特殊字符的函数：

```example
$ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $
```

上面的例子用 [`cases`]($func/math.cases) 函数来描述 f。在 cases 函数中，参数用逗号来分隔，参数也被解释为数学。
如果你需要将参数解释为Typst值，用`#`号作为前缀：

```example
$ (a + b)^2
  = a^2
  + text(fill: #maroon, 2 a b)
  + b^2 $
```

你可以在数学模式内使用所有Typst函数，并插入任何内容。
如果你想让它们正常工作，在参数列表中使用代码模式，你可以在它们的调用前加上一个 `#`。
没有人可以阻止你使用矩形或表情符号作为你的变量：

```example
$ sum^10_(🥸=1)
  #rect(width: 4mm, height: 2mm)/🥸
  = 🧠 maltese $
```

如果你希望直接以 Unicode 形式输入数学符号，也是可以的。

数学调用可以有二维参数列表，使用 `;` 作为分隔符。这方面最常见的用途是创建矩阵的[`mat`]($func/math.mat)函数：

```example
$ mat(
  1, 2, ..., 10;
  2, 2, ..., 10;
  dots.v, dots.v, dots.down, dots.v;
  10, 10, ..., 10;
) $
```

## 如何获得 "LaTeX外观"？ { #latex-look }
用LaTeX编写的论文有一种无可挑剔的外观。这主要是由于它们的字体、[Computer Modern](https://zh.wikipedia.org/wiki/Computer_Modern)、对齐方式、窄行距和宽边距。

下面是一个示例：
- 设置宽[边距]($func/page.margin)
- 启用[两端对齐]($func/par.justify), [更紧密的行间距]($func/par.leading)
  和[首行缩进]($func/par.first-line-indent)
- 设置[字体]($func/text.font)为 "New Computer Modern"，这是一个适用于文本和[代码块]($func/raw)的 OpenType 变体
- 禁用段落[间距]($func/block.spacing)
- 增加[标题]($func/heading)周围的[间距]($func/block.spacing)

```typ
#set page(margin: 1.75in)
#set par(leading: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show par: set block(spacing: 0.55em)
#show heading: set block(above: 1.4em, below: 1em)
```

这应该是一个很好的起点! 如果你想更进一步，为什么不创建一个可重复使用的模板？

## 与LaTeX相比，Typst目前有哪些不足？ { #limitations }
尽管今天Typst可以成为许多人的LaTeX替代品，但仍有一些功能是Typst不（尚未）支持的。
这里列出了功能，在适用的情况下，包含了可能的变通方法。

- **本地图表和绘图。** LaTeX用户经常在PGF/TikZ中与他们的文档一起创建图表。
  Typst还不包括绘制图表的工具，但社区正在加紧提供解决方案，如[`typst-canvas`](https://github.com/johannes-wolf/typst-canvas)、
  [`typst-plot`](https://github.com/johannes-wolf/typst-plot)和[`circuitypst`](https://github.com/fenjalien/circuitypst)。
  你可以把这些工具添加到你的文档中，开始画图。

- **在没有分页符的情况下改变页边距。** 在LaTeX中，页边距总是可以调整的，即使没有分页符。
  要在Typst中改变页边距，你要使用[`page`]($func/page)函数，它将强制分页。
  如果你只想让几个段落伸进页边距，然后再恢复到旧的页边距，你可以使用带负数填充的[`pad`]($func/pad)函数。

- **浮动图。** LaTeX的`figure`命令会聪明地选择放置图片的位置，无论是在页面的顶部或底部，还是在专门的图片页面。
  Typst中的图片将总是出现在它们被插入标记的位置上。虽然这种行为可以省去一些麻烦，但手动放置数字往往是很麻烦的。
  我们将很快增加这一功能!

- **将PDF作为图像。** 在LaTeX中，插入PDF或EPS文件的矢量图已经成为一种习惯。
  Typst不支持这两种格式作为图像格式，但你可以用[在线工具](https://cloudconvert.com/pdf-to-svg)
  或[Inkscape](https://inkscape.org/)轻松地将这两种文件转换成SVG文件。我们计划在Typst的Web应用程序中也加入这些文件格式的自动转换功能。

- **分页符优化。** LaTeX运行一些智能算法，不仅优化换行，也优化换页。虽然Typst试图避免寡头和孤儿，但它使用不太复杂的算法来确定分页符。
  你可以在提交文档前使用`[#pagebreak(weak: true)]`在Typst中插入自定义分页。
  参数`weak`确保不会创建双倍的分页符，如果这个地方无论如何都会是一个自然的分页符。
  你也可以使用`[#v(1fr)]`来分配页面上的空间。它的工作原理与LaTeX的`\vfill`相当类似。

- **书目是不可定制的。** 在LaTeX中，包`bibtex`、`biblatex`和`natbib`提供了广泛的参考和书目格式。
  你也可以使用自定义的`.bbx`文件来定义你自己的样式。
  Typst目前只支持一小部分引文风格，但我们希望在此基础上支持[引文风格语言（CSL）](https://citationstyles.org)，
  这是一种由Zotero支持的基于XML的格式，允许你描述自己的书目风格。
