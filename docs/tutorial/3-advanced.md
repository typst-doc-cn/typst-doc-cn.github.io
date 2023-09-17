---
description: Typst 的教程。
---

# 高级样式
在本教程的前两章中，您学习了如何在 Typst 中编写文档以及如何更改其格式。
你在这两章写的报告得到了极佳的评价，你的导师想以此为基础写一篇会议论文！
当然，这篇报告必须遵循会议的论文样式规范。让我们看看应该如何实现这一目标。

在开始之前，请先让我们创建一个团队，邀请您的导师并让他们加入到团队中。
您可以通过回到 App 的 Dashboard（编辑器左上角的四圈图标）来进行这个操作。
然后，选择左侧工具栏中的加号图标并创建一个团队。
最后，单击新团队并通过单击团队名称旁边的 “管理团队” 转到其设置。
现在，您可以通过电子邮件邀请您的导师了。

![The team settings](3-advanced-team-settings.png)

接下来，将您的项目移动到团队中：
打开项目，通过选择左侧工具栏中的齿轮图标，并从所有者列表中选择您的新团队。
不要忘记保存您的更改！

现在，您的导师也可以编辑项目，并且你们都可以实时查看更改。
您可以加入我们的 [Discord server](https://discord.gg/2uDybryKPe) 查找具有预览访问权限的其他人，
并与他们一起尝试团队功能！

## 会议规范 { #guidelines }
会议布局规范可在会议网站上找到。让我们来看一下：

- 字体应为 11pt 的衬线字体
- 标题应为 17pt 的粗体
- 论文包含单栏摘要和两栏正文
- 摘要应居中
- 正文应对齐
- 第一级章节标题应为 13pt，居中并以小写字母呈现
- 二级标题是短标题，斜体，与正文文本具有相同的大小
- 最后，页面尺寸应为 US letter，编号在页脚的中心，每页的左上角应包含论文的标题

这些要求的大部分我们已经知道应该如何实现了，但对于其中的少部分内容，我们需要学习一些新的技巧。

## 编写正确的 Set 规则 { #set-rules }
让我们从为文档编写一些 Set 规则开始。

```example
#set page(
>>>  margin: auto,
  paper: "us-letter",
  header: align(right)[
    A fluid dynamic model for
    glacier flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Linux Libertine",
  size: 11pt,
)

#lorem(600)
```

你对这里的大部分内容已经很熟悉了。
我们将文本大小设置为 `{11pt}`，将字体设置为 Linux Libertine。
我们还启用了段落对齐，并将页面尺寸设置为 US letter。

`header` 参数是新出现的：
有了它，我们可以使用填入的内容块来填充每个页面的上边距。
在标题中，我们根据会议样式规范的要求将其指定为论文的标题。
我们使用 `align` 函数将文本向右对齐。

最后还有 `numbering` 参数。
在这里，我们可以提供一个 [numbering pattern]($numbering) 来定义如何对页面进行编号。
通过设置为 `{"1"}`，Typst 仅显示最简单的页码。
将其设置为 `{"(1/1)"}` 将显示当前页面和用括号括起来的页总数。
我们甚至可以在这里提供一个完全自定义的函数来实现我们喜欢的内容显示方式。

## 创建标题和摘要 { #title-and-abstract }
现在，让我们添加标题和摘要，我们将从标题开始。
我们将其居中对齐，并通过将其括在 `[*星号*]` 中以将其加粗。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Linux Libertine", 11pt)
#align(center, text(17pt)[
  *A fluid dynamic model
  for glacier flow*
])
```

这看起来是正确的。
我们使用 `text` 函数以覆盖掉之前对 `text` 应用的 Set 规则，将 `text` 函数参数中的 `size` 增加到 17pt。
让我们同时添加作者列表：由于我们是与我们的导师一起撰写这篇论文的，我们将添加我们自己和他们的名字。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Linux Libertine", 11pt)
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)
```

两个作者块彼此相邻，我们使用 [`grid`]($grid) 函数来创建这种布局。
使用 `grid`，我们可以准确控制每列的大小以及哪些内容放到哪个单元格。
`columns` 参数接受 [relative lengths]($relative) 或 [fractions]($fraction) 的数组。
在本例中，我们向它传递了两个相等的 `fractions`，告诉它将可用空间分成两个相等大小的列。
然后，我们将两个内容块参数传递给 `grid` 函数。
第一个是我们自己的信息，第二个是我们的导师的信息。
我们再次使用 `align` 函数将列内的内容居中。
网格采用任意数量的内容块参数来指定单元格。
行是自动添加的，但也可以使用 `rows` 参数手动调整它们的大小。

现在，让我们添加加入。请记住，会议希望摘要居中，且左右不对齐。

```example:0,0,612,317.5
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A fluid dynamic model for
>>>     glacier flow
>>>   ],
>>>   numbering: "1",
>>> )
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
<<< ...

#align(center)[
  #set par(justify: false)
  *Abstract* \
  #lorem(80)
]
>>> #lorem(600)
```

干的漂亮！值得注意的是，我们在 `align` 的内容参数中使用了一个新的 Set 规则来禁用摘要的对齐。
即便它是在第一个 Set 规则之后指定的，这也不会影响文档的其余部分。这是因为内容块拥有 _局部作用域_ 样式。
内容块中设置的任何规则只会影响该内容块中的内容。

另一个调整可以是将论文标题保存在一个变量中，这样我们就不必多次输入标题。
我们可以使用 `{let}` 关键字来做到这一点：

```example:single
#let title = [
  A fluid dynamic model
  for glacier flow
]

<<< ...

>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
#set page(
>>>   "us-letter",
>>>   margin: auto,
  header: align(
    right + horizon,
    title
  ),
<<<   ...
>>>   numbering: "1",
)

#align(center, text(17pt)[
  *#title*
])

<<< ...

>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #lorem(600)
```

将内容绑定到 `title` 变量后，我们可以在函数和标记模式中使用它（前缀为 `#`，就像函数一样）。
这样，如果我们决定修改标题内容，我们可以很轻松地只在一个地方更改它，让后所有出现标题的地方都会同步更新。

## 添加列和标题 { #columns-and-headings }
不幸的是，上面的论文看起来像一堵铅墙。
为了解决这个问题，让我们添加一些标题，并将我们的论文切换到双列布局。
[`columns`]($columns) 函数接受一个整数参数和一个内容参数，并将内容布局到指定数量的列中。
由于我们希望摘要之后的所有内容都在两列中，因此我们需要将 `columns` 函数应用于整个文档。

并不需要将整个文档包装在一个巨大的函数调用里，我们可以使用 “所有内容” Show 规则。
要编写这样的 show 规则，请在 show 关键字后面直接放置一个冒号，然后提供一个函数，该函数将文档的其余部分作为参数输入。
我们在这里将参数称为 `rest`，但您可以自由选择任何参数名。
然后，该函数可以对该内容执行任何操作。在我们的例子中，它将内容传递给 `columns` 函数。

```example:single
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>> #v(4mm)
<<< ...

#show: rest => columns(2, rest)

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

现在只剩下一件事要做了：设置标题样式。
我们需要使它们居中并使用小标题。
因为 `heading` 函数没有提供任何的方法来让我们完成这个任务，
所以我们需要编写自己的标题 Show 规则。

```example:50,250,265,270
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
#show heading: it => [
  #set align(center)
  #set text(12pt, weight: "regular")
  #block(smallcaps(it.body))
]

<<< ...
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #v(4mm)
>>> #show: rest => columns(2, rest)
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

这看起来很棒！我们使用了适用于所有标题的 Show 规则。
我们给了它一个函数，该函数将标题作为参数传递。
该参数可以用作内容块，但它也有一些字段，如 `title`、`numbers` 和 `level`，我们可以使用它们组成我们需要的自定义外观。
在这里，我们应用居中对齐，并将字体粗细设置为 `{"regular"}`，因为标题默认为粗体，
并使用 [`smallcaps`]($smallcaps) 函数以小写字母的方式呈现标题。

唯一剩下的问题是，现在所有标题看起来都一模一样。
“Motivation” 和 “Problem Statement” 子小节应该是斜体标题，但现在，它们看起来与小节标题没有区别。
我们可以通过在设置规则上使用 `where` 选择器来解决这个问题：
这是一个 [method]($scripting/#methods)，我们可以标题（和其他元素）上调用，允许我们按 `level` 过滤它们。
我们可以用它来区分小节和子小节标题：

```example:50,250,265,245
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
>>>
#show heading.where(
  level: 1
): it => block(width: 100%)[
  #set align(center)
  #set text(12pt, weight: "regular")
  #smallcaps(it.body)
]

#show heading.where(
  level: 2
): it => text(
  size: 11pt,
  weight: "regular",
  style: "italic",
  it.body + [.],
)
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #v(4mm)
>>> #show: rest => columns(2, rest)
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

这看起来很棒！我们编写了两个显示规则，每个规则都有选择地应用于一级和二级标题。
我们使用 `where` 选择器按级别过滤标题。然后，我们将小节标题呈现为短标题。
我们还会自动在子小节标题的末尾添加一个点号。

让我们回顾一下会议的样式规范：
- 字体应为 11pt 衬线字体 ✓
- 标题应为 17pt 和粗体 ✓
- 论文包含单栏摘要和两列正文 ✓
- 摘要应居中 ✓
- 正文应对齐 ✓
- 第一级章节标题应居中，以小写字母和 13pt 呈现 ✓
- 二级标题是短标题， 斜体，大小与正文相同 ✓
- 最后，页面尺寸应为 US letter，编号在中心，每页的左上角应包含论文标题 ✓

我们现在符合所有这些规范，可以向会议提交论文了！完成的论文如下所示：

<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## 回顾 { #review }
您现在已经学习了如何创建页眉和页脚，如何使用函数和作用域范围在本地覆盖样式，
如何使用 [`grid`]($grid) 函数创建更复杂的布局，
以及如何为单个函数和整个文档编写 Show 规则。
您还学习了如何使用 [`where` 选择器]($styling/#show-rules) 按级别过滤标题。

这篇论文取得了巨大的成功！
你在会议上遇到了很多志同道合的研究人员，并计划了一个项目，你希望明年在同一地点发表。
不过，您需要使用相同的样式规范撰写一篇新论文，
所以也许现在您想为您和您的团队创建一个能够节省你们时间的模板？

在下一节中，我们将学习如何创建可在多个文档中复用的模板。
这是一个更高级的主题，所以如果你现在觉得并没有必要了解，可以以后再学习。
