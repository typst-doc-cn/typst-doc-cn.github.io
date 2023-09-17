---
description: Typst 中所有有关文档样式设置概念。
---

# 样式

Typst 有一个自由的样式设置系统，可以按需自动格式化文档。
_set 规则_ 可以配置文档元素的基本属性，用来设置大多常用文档样式，
但是有些样式设置并没有属性可设，因此 Typst 引入 _show 规则_，进而可以彻底的重设文档元素外观。

## set 规则 { #set-rules }

使用 set 规则，可以自定义文档元素的外观，
这些规则以 `{set}` 关键字作为开始标记（在标记模式下使用 `[#set]`），
紧随一个文档元素的[函数调用]($function)，
set 函数调用时，仅有特定参数可被允许使用，可以参考该函数文档查看有那些参数可以用于 set 规则。
下面示例，使用了两个 set 规则来改变[文档字体]($text.font)和[标题数字]($heading.numbering)。

```example
#set heading(numbering: "I.")
#set text(
  font: "New Computer Modern"
)

= Introduction
With set rules, you can style
your document.
```

顶层 set 规则一直作用到文件结束，当在块内使用时，只作用到块结束。
这样使用块，可以限制 set 规则只总用于文档的特定片段。
下面示例使用了文档内容块来限制列表样式设置只作用于特定列表。

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

有时，想要实现特定条件下 set 规则才有效，可以使用 _set-if_ 规则。

```example
#let task(body, critical: false) = {
  set text(red) if critical
  [- #body]
}

#task(critical: true)[Food today?]
#task(critical: false)[Work deadline]
```

## show 规则 { #show-rules }

使用 show 规则可以深度定制特定类型文档元素的外观，最常用的基本形式是 _show-set 规则_，
以 `{show}` 关键字作为开始标记，紧随一个[选择器]($selector)，一个冒号，最后是一个 set 规则。
最常见的选择器是一个[文档元素函数名]($function/#element-functions)，
是 set 规则用来选择相关文档元素的。
下面示例中，标题变为深蓝色，其他文本保持黑色。

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

使用 show-set 规则，可以混搭各种函数属性，来实现各种不同的效果，但这也仍然局限于 Typst 预定义的功能。
为了最大限度的灵活设置，可以使用函数式 show 规则，用来定义如何从 0 开始格式化文档元素。
使用一个 [函数]($function) 来替换 show-set 规则中的 set 规则，
这个函数以未知文档元素作为参数，返回任意文档内容，函数的文档元素参数有各种不同的[属性字段]($scripting/#fields)。
下面示例中用一个函数式 show 规则格式化一个虚构的百科全书标题。

```example
#set heading(numbering: "(I)")
#show heading: it => block[
  #set align(center)
  #set text(font: "Inria Serif")
  \~ #emph(it.body)
     #counter(heading).display() \~
]

= Dragon
With a base health of 15, the
dragon is the most powerful
creature.

= Manticore
While less powerful than the
dragon, the manticore gets
extra style points.
```

与 set 规则类似， show 规则也一直作用到文档或者当前块的结束。

show 规则的右边部分不仅可以是一个函数，也可以是一个字符串常量或者文档内容块，用来直接替换文档元素。
show 规则的左边部分也可以是 _下面类型的选择器_，用来定义哪些文档元素会被转换：

- **所有文档：** `{show: rest => ..}` \
  转换 show 规则后的所有文档元素，这样就免于将所有文档元素都包含在一个巨大的函数调用中，来实现更复杂的布局。

- **特定文本：** `{show "Text": ..}` \
  设置特定文本样式，转变或替换特定文本。

- **正则表达式：** `{show regex("\w+"): ..}` \
  更自由的选择并转换匹配特定正则表达式的文本，详见于 [regex 函数]($regex)文档。

- **字段选择函数：** `{show heading.where(level: 1): ..}` \
  转换具有特定字段的文档元素。比如，可以只设置文档一级标题样式。

- **标签：** `{show <intro>: ..}` \
  选择并转换具有特定标签的文档元素，详见于[标签函数]($label)文档。

```example
#show "Project": smallcaps
#show "badly": "great"

We started Project in 2019
and are still working on it.
Project is progressing badly.
```
