---
description: |
  Typst 参考索引文档是关于 Typst 排版语言的详细文档。
---

# 参考

本参考索引是一个综合指南文档，包括 Typst 的语法，概念，类型和函数。
如果你初识 Typst，建议从 [学习指南]($tutorial) 开始，然后再按需回来学习更多的 Typst 特性。

## 语言 { #language }

本参考索引第一部分粗略介绍 [Typst 语法]($syntax)，
包括 [文档样式]($styling) 概念，以及 [Typst 脚本功能]($scripting) 详细文档。


## 函数 { #functions }

第二部分引入所有能用到的函数，
有插入文档内容的，有设置文档内容样式的，有文档内容变换的，有布局文档内容的。
每一个函数均已同样的形式表述，
有些是表述函数基本功能，有些列举函数参数，有些举例如何使用函数。

参考索引最后一部分描述的 Typst 代码模式内的函数，用来操作及转换数据的。
和第二部分一样，每一个函数均已同样的形式表述，
有些是表述函数基本功能，有些列举函数参数，有些举例如何使用函数。

## 进阶 { #advanced }

译者注：这部分官方文档没有，是译者自主添加的。

除了参考，还可以考虑阅读 [typst-examples-book](https://sitandr.github.io/typst-examples-book/book/)，里面包含了一些 Typst 的高级知识、简单示例，以及一些最佳实践。

例如简单地实现类似 Markdown 中的引用文本样式：

```example
+ #lorem(10) \
  #rect(fill: luma(240), stroke: (left: 0.25em))[
    *Solution:* #lorem(10)

    $ a_(n+1)x^n = 2... $
  ]
```