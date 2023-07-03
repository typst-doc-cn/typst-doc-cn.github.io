---
description: |
  中文用户指南
---

# 中文用户指南
本页面并不属于官方文档的内容，而是 Typst 中文社区所撰写的面向中文用户的指南。
如果您对在 Typst 中使用中文有什么使用心得，也欢迎在参与贡献！

## 常见 Q&A { #question-and-answer }

### 如何为中英文设置不同的字体？

可以使用 text 里面的 fallback 特性。
Typst 中的 font 参数可以接收一个数组，会根据字体里有无当前字符来依次选择字体。
因此我们只需要传入一个英文字体后接中文字体的数组，就可以达到为中英文设置不同的字体的效果。

```example
Hello World 你好世界

#[
  #set text(font: ("IBM Plex Serif", "Noto Sans CJK SC"))

  Hello World 你好世界
]
```


## 一些 Typst 中文资源列表 { #resources }

- 中国大学论文
    - [pkuthss-typst](https://github.com/lucifer1004/pkuthss-typst): 北京大学学位论文模板,Typst template for dissertations in Peking University (PKU).
    - [BUAA-typst](https://github.com/cherichy/BUAA-typst): 北京航空航天大学学位论文模板
    - [bupt-typst](https://github.com/QQKdeGit/bupt-typst): 北京邮电大学本科学士学位论文模板
    - [HUST-typst-template](https://github.com/werifu/HUST-typst-template): 用于华科毕业设计（本科）的 typst 模板。
    - [SHU-Bachelor-Thesis-Typst](https://github.com/shuosc/SHU-Bachelor-Thesis-Typst): 上海大学本科毕业论文 typst 模板 (开发ing)
    - [sysu-thesis-typst](https://github.com/howardlau1999/sysu-thesis-typst): 中山大学学位论文 Typst 模板
    - [ZJGSU-typst-template](https://github.com/jujimeizuo/ZJGSU-typst-template): 浙江工商大学毕业设计（本科）的 typst 模板。
    - [CQUPTypst](https://github.com/jerrita/CQUPTypst): 一个 Typest 模板，但是大专 
    - [zjut-report-typst](https://github.com/zjutjh/zjut-report-typst): 浙江工业大学一些实验报告的 Typst 模板, Some report templates of Zhejiang University of Technology.
    - [HIT-Thesis-Typst](https://github.com/chosertech/HIT-Thesis-Typst): 适用于哈尔滨工业大学学位论文的 Typst 模板
- 简历
    - [uniquecv-typst](https://github.com/gaoachao/uniquecv-typst): 一个使用Typst编写的简历模板，基于uniquecv。
    - [typst-cv-miku](https://github.com/ice-kylin/typst-cv-miku): 简历模板，有多种版本，包括中文 ,This is a simple, elegant, academic style CV template for typst. Support for English and Chinese (and more).
    - [awesomeCV-Typst](https://github.com/mintyfrankie/awesomeCV-Typst) - 一份参考 `Awesome-CV` 的简历模版，支持多语言简历管理, An opinionated, relived CV template inspired by the LaTeX `Awesome-CV` project, but with multilingual support and more
    - [Chinese-Resume-in-Typst](https://github.com/OrangeX4/Chinese-Resume-in-Typst): 使用 Typst 编写的中文简历, 语法简洁, 样式美观, 开箱即用, 可选是否显示照片