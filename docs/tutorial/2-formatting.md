---
description: Typst 的教程。
---

# 格式
到目前为止，您已经写了一份包含一些文本、一些数学公式和图像的报告。
但是，它看起来仍然很朴素。您的助教还不知道您正在使用新的排版系统，
并且您希望您的报告与其他学生提交的报告外观上一致。
在本章中，我们将了解如何使用 Typst 的样式系统来格式化你的报告。

## Set 规则 { #set-rules }
正如我们在上一章中看到的，Typst 具有 _插入_ 内容块的函数（例如 [`image`]($image) 函数），
以及其他将内容块作为参数接收的 _操作_ 函数（例如 [`align`]($align) 函数）。
您可能第一个想法是，例如，为了报告的文本左右对齐（justify），您可能会寻找一个执行此操作的函数并将整个文稿包装在其中。

```example
#par(justify: true)[
  = Background
  In the case of glaciers, fluid
  dynamics principles can be used
  to understand how the movement
  and behavior of the ice is
  influenced by factors such as
  temperature, pressure, and the
  presence of other fluids (such as
  water).
]
```

等等，函数的所有参数不应该在括号内指定吗？为什么在圆括号 _后面_ 有第二组方括号组成的内容块？
答案是，由于将内容块传递给函数在 Typst 中非常常见，因此它有着特殊的语法：
无需将内容块放在参数列表中，而是可以直接将其写在方括号中，并放在普通参数之后，从而节省标点符号。

<div class="info-box">

译者注：这其实只是一个语法糖，即任何 `fn(...)[XXX][YYY][ZZZ]`，都会被自动转成 `fn(..., [XXX], [YYY], [ZZZ])`。

所以你可以对任意一个函数使用，包括你自己的自定义函数，只要在转换后的结果符合函数入参要求即可。

</div>

如上所示，这个语法是有效的。[`par`]($par) 函数左右对齐了里面的所有段落。
但是，将文稿包装在无数的函数中，并选择地就地应用样式，这很快就会变得麻烦且复杂。

幸运的是，Typst 有一个更优雅的解决方案。
使用 _Set 规则_ ，您可以将样式属性应用于某类内容块的所有实例。
通过输入 `{set}` 关键字编写 Set 规则，后面跟随着你要设置属性的函数的名称，
并在括号中输入你需要的新默认参数列表。

```example
#set par(justify: true)

= Background
In the case of glaciers, fluid
dynamics principles can be used
to understand how the movement
and behavior of the ice is
influenced by factors such as
temperature, pressure, and the
presence of other fluids (such as
water).
```

<div class="info-box">

想以更深层的方式了解这里发生了什么吗？

Set 规则可以概念化为，为将来该函数的所有使用的某些参数设置默认值。
</div>

## 自动补全面板 { #autocomplete }
如果您按照操作并在 App 中尝试了一些操作，您可能已经注意到，
在输入 `#` 字符后，总是会弹出一个面板，向您显示可用函数，并在参数列表中显示可用参数。
这是自动补全面板。它在编写文档时非常有用：您可以通过按 Return 键或使用箭头键导航到所需的补全来应用该建议。
面板可以通过按 Esc 键关闭，然后通过输入 `#` 或按 <kbd>Ctrl</kbd> + <kbd>Space</kbd> 再次打开。
使用自动补全面板去掌握函数的正确参数。大多数建议都附有对它们所做的事情的简短描述。

![Autocomplete panel](2-formatting-autocomplete.png)

## 设置页面 { #page-setup }
回到 Set 规则：编写规则时，您可以根据要设置样式的元素类型来选择函数。
以下是 Set 规则中常用的一些函数的列表：

- [`text`]($text) 用于设置文本的字体、大小、颜色和其他属性
- [`page`]($page) 用于设置页面大小、边距、页眉、启用栏和页脚
- [`par`]($par) 用于对齐段落、设置行距等
- [`heading`]($heading) 用于设置标题的外观与启用编号
- [`document`]($document) 用于设置 PDF 输出中包含的元数据，例如标题和作者

并非所有函数参数都可以设置。
通常，只能设置告诉函数 _如何_ 做某事的参数，而不能设置告诉函数 _做什么_ 的参数。
函数参考页指明了哪些参数是可以应用 Set 规则的。

让我们向文档添加更多样式，我们想要更大的边距和衬线字体。
出于示例的目的，我们还将设置另一个页面大小。

```example
#set text(
  font: "New Computer Modern",
  size: 10pt
)
#set page(
  paper: "a6",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set par(
  justify: true,
  leading: 0.52em,
)

= Introduction
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behavior of these natural structures.

>>> Glacier displacement is influenced
>>> by a number of factors, including
>>> + The climate
>>> + The topography
>>> + The geology
>>>
>>> This report will present a physical
>>> model of glacier displacement and
>>> dynamics, and will explore the
>>> influence of these factors on the
>>> movement of large bodies of ice.
<<< ...

#align(center + bottom)[
  #image("glacier.jpg", width: 70%)

  *Glaciers form an important
  part of the earth's climate
  system.*
]
```

这里有几点需要注意。

首先是 [`page`]($page) 的 Set 规则，它接收两个参数：页面大小和页面边距。
页面大小为字符串，Typst 接受 [许多标准页面大小]($page.paper)，但您也可以指定自定义页面大小。
边距为一个 [字典]($dictionary)，字典是键值对的集合。
在本例中，键为 `x` 和 `y`，值分别为水平边距和垂直边距。
我们还可以通过传递带有键 `{left}`、`{right}`、`{top}` 和 `{bottom}` 的字典来为每边指定单独的边距。

其次是 [`text`]($text) 的 Set 规则。
在这里，我们将字体大小设置为 `{10pt}`，将字体设置为 `{"New Computer Modern"}`。
Typst App 带有许多字体，您可以在您的文档自主尝试。
当您在输入 `text` 函数的 `font` 参数时，您可以在自动补全面板中发现所有可用的字体。

我们还设置了行间距（又名行距）：它被指定为 [length]($length) 值，
我们使用 `em` 单位来指定相对于字体大小的行距：`{1em}` 相当于当前字体大小（默认为 `{11pt}`）。

最后，我们通过加入中心对齐和垂直对齐来对图像进行底部对齐。
垂直和水平对齐可以与 `{+}` 运算符结合使用，以生成 2D 对齐。


## 更复杂一点 { #sophistication }
为了更清楚地组织我们的文档，我们现在要对标题进行编号。
我们可以通过设置 [`heading`]($heading) 函数的 `numbering` 参数来做到这一点。

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

我们指定了字符串 `{"1."}` 作为编号参数。
这将告诉 Typst 用阿拉伯数字对标题进行编号，并在每个级别的编号之间放置一个点。
我们还可以使用 [字母，罗马数字和符号]($numbering) 作为编号：

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.a")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

此示例还使用 [`lorem`]($lorem) 函数生成一些占位文本。
此函数将一个数字作为参数，并生成许多 _Lorem Ipsum_ 文本单词。

<div class="info-box">

您是否想知道为什么标题和文本 Set 规则适用于所有文本和标题，即使它们不是使用函数生成的？

Typst 每次在你写  `[= Conclusion]` 时都会在内部调用 `heading` 函数。
实际上，函数调用 `[#heading[Conclusion]]` 等效于上面的标题标记。
其他标记元素的工作方式类似，它们仅仅是相应的函数调用的 _语法糖_ 。
</div>

## Show 规则 { #show-rules }
你已经对这个结果很满意了。
但最后一件事需要修改：您正在编写的报告是为一个更大的项目准备的，
在该项目的名称旁始终应该附上项目的 Logo，即使是仅有文字的单调文章。

你在考虑你的选择。您可以使用搜索和替换在 Logo 的每个实例之前添加 `[#image("logo.svg")]` 调用，这听起来很乏味。
相反，你可以 [定义一个自定义函数]($function/#definitions)，它将生成带有图像的 Logo。但是，还有一种更简单的方法：

使用 Show 规则，您可以重新定义 Typst 显示某些元素的方式。
您可以指定 Typst 应以不同的方式显示哪些元素以及它们的外观。
显示规则可以应用于文本实例、许多函数，甚至整个文档。

```example
#show "ArtosFlow": name => box[
  #box(image(
    "logo.svg",
    height: 0.7em,
  ))
  #name
]

This report is embedded in the
ArtosFlow project. ArtosFlow is a
project of the Artos Institute.
```

在这个例子中有很多新的语法：我们写入 `{show}` 关键字，后面跟一个我们希望以不同方式显示的文本字符串，以及一个冒号。
然后，我们编写一个函数，该函数将应显示的内容作为参数输入，在这里，我们称该参数为 `名称`。
我们现在可以使用函数体中的 `name` 变量来输出名称 `ArtosFlow`。
我们的 Show 规则在名称前面添加 Logo 图像，并将结果放入 `box` 中，以防止 Logo 和名称之间出现换行符。
图像也放在一个 `box` 中，这样它就不会出现在自己的段落中。

对第一个 `box` 函数和 `image` 函数的调用不需要前导 `#`，因为它们没有直接嵌入到标记文本中。
当 Typst 处于代码模式而不是标记模式时，不需要前导 `#` 来访问函数、关键字和变量。
同样的现象也可以在函数参数列表、函数定义和 [代码块]($scripting) 中观察到。


## 回顾 { #review }
您现在知道如何将基本格式应用于 Typst 文档。
您学习了如何设置字体、对齐段落、更改页面尺寸以及使用 Set 规则向标题添加编号。
您还学习了如何使用基本的 Show 规则来更改文本在整个文档中的显示方式。

您提交了报告。您的导师对此非常满意，他们想将其改编成会议论文！
在下一节中，我们将学习如何使用更高级的 Show 规则和函数将文档格式化为论文。