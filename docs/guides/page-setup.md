---
description: |
  在 Typst 中设置页面尺寸、边距和页码的深入指南。
  了解如何创建吸引人且清晰的布局并快速实现目标。
---

# 页面设置指南
你的页面设置在文档的第一印象中起着重要作用。
行长度、页边距和栏目布局影响着文档的[外观](https://practicaltypography.com/page-margins.html)和[易读性](https://designregression.com/article/line-length-revisited-following-the-research)，而合适的页眉、页脚可以帮助读者轻松地导航文档。
本指南将帮助您自定义页面、页边距、页眉、页脚和页码，使其与你的内容完美匹配，让你能够开始写作。

在 Typst 中，每个页面都有宽度、高度以及四个方向上的页边距。
顶部和底部的页边距可以包含页眉和页脚。
页面元素的设置规则是你控制页面设置的地方。
如果你在这个设置规则中进行了更改，Typst 会确保在之后插入一个新的符合规范的空白页面，因此可能会插入分页符。
因此，最好在文档开始或模板中指定你的 ['{page}`]($page) 设置规则。

```example
#set rect(
  width: 100%,
  height: 100%,
  inset: 4pt,
)
>>> #set text(6pt)
>>> #set page(margin: auto)

#set page(
  paper: "iso-b7",
  header: rect(fill: aqua)[Header],
  footer: rect(fill: aqua)[Footer],
  number-align: center,
)

#rect(fill: aqua)
```

这个示例可视化了页面内容、页眉和页脚的尺寸。
页面内容是页面尺寸（ISO B7）减去每个边的默认页边距。
在顶部和底部边距中，用绘制的矩形来可视化页眉和页脚。
它们不接触主要内容，而是通过各自边距的 30% 偏移。
你可以通过指定 [`header-ascent`]($page.header-ascent) 和 [`footer-descent`]($page.footer-descent) 参数来控制这个偏移量。

接下来，本指南将更详细地介绍如何通过示例来满足常见的页面设置要求。

## 自定义页面尺寸和页边距 { #customize-margins }
Typst 的默认页面大小是 A4 纸张。
根据你所在的地区和使用情况，你可能希望进行更改。
你可以通过使用 [`{page}`]($page) 设置规则，并传递一个字符串参数来使用常见的页面大小来实现这一点。
选项包括完整的 ISO 216 系列（例如 `"iso-a4"`、`"iso-c2"`）、美国习惯格式如 `"us-legal"` 或 `"us-letter"`，以及其他选项。
查阅有关 [page 的 paper 参数]($page.paper)的参考文档，了解所有可用选项。

```example
>>> #set page(margin: auto)
#set page("us-letter")

This page likes freedom.
```

如果你需要根据特定尺寸自定义页面大小，可以使用命名参数 [`width`]($page.width) 和 [`height`]($page.height) 进行指定。

```example
>>> #set page(margin: auto)
#set page(width: 12cm, height: 12cm)

This page is a square.
```

### 更改页边距 { #change-margins }
边距是一个良好排版的重要组成部分：[排版师认为每行容纳 45 到 75 个字符的长度最适合易读性](http://webtypography.net/2.1.2)，而边距和[栏目](#columns)则有助于定义行宽。
默认情况下，Typst 将根据文档的页面大小创建比例适当的边距。
要设置自定义边距，你可以在 [`{page}`]($page) 设置规则中使用 [`margin`]($page.margin) 参数。

如果你想将所有边距设置为相同的宽度，`margin` 参数接受一个长度值。
然而，通常情况下你可能希望在每个边上设置不同的边距。
为了实现这一点，你可以传递一个字典：

```example
#set page(margin: (
  top: 3cm,
  bottom: 2cm,
  x: 1.5cm,
))

#lorem(100)
```

页边距字典可以有每个边的键（`top`、`bottom`、`left`、`right`），但你也可以通过设置页边距字典的 `x` 键来同时控制左右边距，就像示例中所示。同样地，通过设置 `y` 键，可以同时调整顶部和底部边距。

如果在页边距字典中没有为所有边指定边距，那么未设置的边将保持原有的边距设置。
为了防止这种情况，并将所有剩余的边距设置为相同的大小，你可以使用 `rest` 键。
例如，`[#set page(margin: (left: 1.5in, rest: 1in))]` 将把左边距设置为1.5英寸，其余边距设置为1英寸。

### 在交替的页面上设置不同的边距 { #alternating-margins }
有时候，你需要在奇偶页之间交替设置水平边距，例如，在书籍的内侧（靠近书脊）需要更多的空间，而在页面的外侧需要较小的边距。
Typst 会跟踪每个页面是在书脊的左侧还是右侧。
你可以利用这个信息，并设置边距字典的 `inside` 或 `outside` 键。
`inside` 边距是指向书脊的边距，`outside` 边距是指向装订书籍边缘的边距。

```typ
#set page(margin: (inside: 2.5cm, outside: 2cm, y: 1.75cm))
```

Typst 假设从左到右书写的文档左侧装订，而从右到左书写的书籍右侧装订。
然而，在某些情况下，你需要进行更改：如果你的第一页由其他应用程序生成，则从Typst的角度来看，装订方式会相反。
此外，一些书籍，如英语漫画，尽管使用从左到右的写作顺序，但习惯上是右侧装订。
为了改变装订的一侧并明确设置 `inside` 和 `outside` 的位置，你可以在 [`{page}`]($page) 设置规则中使用 [`binding`]($page.binding) 参数。

```typ
// 尽管是用西班牙语编写的，但是制作一本右侧装订的书籍。
#set text(lang: "es")
#set page(binding: right)
```

如果 `binding` 参数设置为 `left`, 则在奇数页上，`inside` 边距将位于左侧，而偶数页上则相反。

## 添加页眉和页脚 { #headers-and-footers }
每个页面的顶部和底部边距中都可以插入页眉和页脚。
你可以添加自定义的页眉和页脚，或者只插入页码。

如果你需要更多内容而不仅仅是页码，最好的方法是使用 [`{page}`]($page) 设置规则中的 [`header`]($page.header) 和 [`footer`]($page.footer) 参数来插入页眉和页脚。你可以传递任何内容作为它们的值：

```example
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: [
  _Lisa Strassner's Thesis_
  #h(1fr)
  National Academy of Sciences
])

#lorem(150)
```

默认情况下，页眉是底部对齐的，以避免与页面顶部边缘发生冲突。
如果你想修改对齐方式，可以将页眉内容包裹在 [`{align}`]($align) 函数中。

### 在特定页面上使用不同的页眉和页脚 { #specific-pages }
你可能需要在某些页面上使用不同的页眉和页脚。
例如，你可能不希望在标题页上显示页眉和页脚。
下面的示例展示了如何根据条件在第一页上移除页眉：

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: locate(loc => {
  if counter(page).at(loc).first() > 1 [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
}))

#lorem(150)
```

这个示例可能看起来有点复杂，但我们来逐步解释一下：我们告诉 Typst 页眉取决于当前的[位置]($locate)。
`loc` 值允许其他函数了解我们当前所在页面的位置。
然后，我们询问 Typst 当前位置的页面计数器是否大于 1。
页面计数器从 1 开始，所以我们在只有一页的情况下跳过页眉。
计数器可以有多个级别。
这个功能用于类似标题的项目，但页面计数器始终只有一个级别，所以我们只需要查看第一个级别。

当然，你可以在这个示例中添加一个 `else` 语句，以在第一页上添加不同的页眉。

### 根据特定元素在页面上调整页眉和页脚 { #specific-elements }
先前指南中描述的一种技术，可以根据 Typst 的标签执行更高级的任务。
例如，具有大型表格的页面可以省略页眉，以减少混乱。
我们可以使用 `<big-table>` [标签]($label)来标记我们的表格，并使用[查询系统]($query)来判断当前页面是否存在这样的标签：

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: locate(loc => {
  let page-counter = counter(page)
  let matches = query(<big-table>, loc)
  let current = page-counter.at(loc)
  let has-table = matches.any(m =>
    page-counter.at(m.location()) == current
  )

  if not has-table [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
}))

#lorem(100)
#pagebreak()

#table(
  columns: 2 * (1fr,),
  [A], [B],
  [C], [D],
) <big-table>
```

在这个示例中，我们查询所有 `<big-table>` 标签的实例。
然后，我们检查当前位置的页面上是否有任何表格。
如果没有，则打印页眉。
这个示例还使用了变量以使代码更简洁。
就像之前一样，你可以添加一个 `else` 语句来添加另一个页眉，而不是删除它。

## 添加和自定义页码 { #page-numbers }
页码可以帮助读者更轻松地跟踪和引用你的文档。
插入脚注的最简单方法是使用 [`{page}`]($page) 设置规则的 [`numbering`]($page.numbering) 参数。
你可以传递一个表示页码格式的[_编号模式_]($numbering.numbering)字符串。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1")

This is a numbered page.
```

在上面的示例中，你可以看到最简单的示例。
它在页脚的中央添加了一个阿拉伯数字的页码。
你可以指定除了 `"1"` 之外的其他字符来获取其他数字形式。
例如，使用 `"i"` 将生成小写的罗马数字。
任何不被解释为数字的字符都将按原样输出。
例如，要在页码周围加上破折号，可以输入以下内容：

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "— 1 —")

This is a — numbered — page.
```

你可以通过在字符串中添加第二个数字字符来添加总页数。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1 of 1")

This is one of many numbered pages.
```

要了解可以在此处传递的参数，请查阅 [`{numbering}`]($numbering.numbering) 函数的参考文档。

如果你需要将页码右对齐或左对齐，可以使用 [`{page}`]($page) 设置规则的 [`number-align`]($page.number-align) 参数。
然而，请注意，目前无法使用此属性实现偶数页和奇数页之间的交替对齐。
要实现这一点，你需要指定一个自定义的页脚，并按照在有关有条件省略页眉和页脚的部分中描述的方式查询页面计数器。

### 自定义带页码的页脚
有时候，你需要在页脚中添加除了页码以外的其他内容。
然而，一旦指定了页脚，[`{page}`]($page) 设置规则的 [`numbering`]($page.numbering) 参数将被忽略。
本节将向你展示如何添加带有页码和其他内容的自定义页脚。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: [
  *American Society of Proceedings*
  #h(1fr)
  #counter(page).display(
    "1/1",
    both: true,
  )
])

This page has a custom footer.
```

首先，我们在左侧添加了一些强调文本，并添加了自由空间来填充行。
然后，我们调用 `counter(page)` 来获取页面计数器，并使用其 `display` 函数来显示当前值。
我们还将 `both` 设置为 `{true}`，以便我们的编号模式适用于当前页码和最终页码。

我们还可以对页码进行更有创意的处理。
例如，让我们为每个页面插入一个圆圈。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: [
  *Fun Typography Club*
  #h(1fr)
  #counter(page).display(num => {
    let circles = num * (
      box(circle(
        radius: 2pt,
        fill: navy,
      )),
    )
    box(
      inset: (bottom: 1pt),
      circles.join(h(1pt))
    )
  })
])

This page has a custom footer.
```

在这个示例中，我们根据页面数量创建了一个包含[圆圈]($circle)的数组。
圆圈被包装在一个 [box]($box) 中，这样它们就可以出现在同一行上，因为它们是块级元素，否则会创建段落换行。
该数组的长度取决于当前页码。

然后，我们将圆圈插入到页脚的右侧，并在它们之间留出 1pt 的空间。
数组的 [`join`]($scripting/#blocks) 方法将尝试将数组的不同值连接成一个单一的值，并以其参数为间隔插入。
在我们的示例中，我们获得了一个带有圆圈和它们之间空格的单一内容值，我们可以将其与 `align` 函数一起使用。
最后，我们使用另一个 `box` 来确保文本和圆圈可以共享一行，并使用 [`inset`]($box.inset) 参数将圆圈稍微提高，以便与文本对齐。

### 重置页码或跳过页码 { #skip-pages }
你是否需要在文档中的某个地方重置页码？
也许你想在标题页之后开始。或者你需要跳过一些页码，因为你将在最终的打印产品中插入页面。

正确修改页码的方式是操作页面计数器 [`counter`]($counter)。
最简单的操作是将计数器设置回 1。

```typ
#counter(page).update(1)
```

这行代码将页码计数器重置为 1。
它应该放置在页面的开头，否则会创建一个页面分隔。
你也可以通过传递一个函数来更新计数器，给定其先前的值：

```typ
#counter(page).update(n => n + 5)
```

在这个示例中，我们跳过了五页。`n` 是页面计数器的当前值，`n + 5` 是我们函数的返回值。

如果你需要获取实际的页码而不是页面计数器的值，你可以在 `locate` 闭包的参数上使用 [`{page}`]($page) 方法：

```example
#counter(page).update(n => n + 5)

// This returns one even though the
// page counter was incremented by 5.
#locate(loc => loc.page())
```

你还可以通过 `locate` 闭包参数使用 [`page-numbering`]($locate) 方法获取页码编号模式。

## 添加栏 { #columns }
要在文档中保持易读行长度的同时适应更多内容，请使用栏。
栏是由一些空白分隔的垂直文本块。
这个空白区域被称为 gutter（装订线）。

如果你的所有内容都需要以栏的方式布局，你可以在 [`{page}`]($page.columns) 设置规则中指定所需的栏数：

```example
>>> #set page(height: 120pt)
#set page(columns: 2)
#lorem(30)
```

如果你需要调整栏之间的装订线，请参考下一节中使用的方法。

### 在文档的任何位置使用栏 { #columns-anywhere }
在科学论文中，非常常见的是标题和摘要以单栏形式呈现，而正文以双栏形式呈现。
为了实现这种效果，Typst 包含了一个独立的 [`{columns}`]($columns) 函数，可以在页面的任何位置插入栏。

从概念上讲，`columns` 函数必须包含列的内容：

```example:single
>>> #set page(height: 180pt)
= Impacts of Odobenidae

#set par(justify: true)
>>> #h(11pt)
#columns(2)[
  == About seals in the wild
  #lorem(80)
]
```

然而，我们可以使用 ["everything show rule"]($styling/#show-rules) 来减少嵌套并编写更易读的 Typst 标记：

```example:single
>>> #set page(height: 180pt)
= Impacts of Odobenidae

#set par(justify: true)
>>> #h(11pt)
#show: columns.with(2)

== About seals in the wild
#lorem(80)
```

显示规则将会将其后的所有内容包裹在它的函数中。
[`with`]($function.with) 方法允许我们向函数传递参数，本例中是栏数，而无需调用该函数。

`columns` 函数的另一个用法是在类似矩形的容器内创建栏，或者自定义装订线的大小：

```example
#rect(
  width: 6cm,
  height: 3.5cm,
  columns(2, gutter: 12pt)[
    In the dimly lit gas station,
    a solitary taxi stood silently,
    its yellow paint fading with
    time. Its windows were dark,
    its engine idle, and its tires
    rested on the cold concrete.
  ]
)
```

### 平衡栏的长度
如果文档的最后一页上的栏长度差异很大，可能会产生不平衡和不吸引人的布局。
这就是为什么排版师经常会在最后一页上平衡栏的长度。
这种效果称为平衡栏。
Typst 目前无法自动平衡栏。
然而，你可以通过在你的标记中适当的位置放置 [`[#colbreak()]`]($colbreak) 来手动平衡栏，从而手动创建所需的栏分隔。


## 单次修改
如果你只需要插入一个具有不同设置的单独页面，就无需覆盖整个页面设置。
例如，你可能想插入一个横向页面以插入一个大表格，或者为标题页更改边距和列数。
在这种情况下，你可以将 [`{page}`]($page) 作为一个带有内容作为参数和其他参数为覆盖设置的函数进行调用。
这将插入足够多的新页面，并使用你的覆盖设置来放置你的内容。
在调用结束后，Typst 将恢复到设置规则中的页面设置。

```example
>>> #set page("a6")
#page(flipped: true)[
  = Multiplication table

  #table(
    columns: 5 * (1fr,),
    ..for x in range(1, 6) {
      for y in range(1, 10) {
        (str(x*y),)
      }
    }
  )
]
```
