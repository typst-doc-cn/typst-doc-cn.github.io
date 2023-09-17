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

### 为什么我设置的字体没有生效？

如果中文字体不符合typst要求，那么它不会选择你声明的字体，例如字体的变体数量不够等原因。

1. `typst fonts` 查看系统字体，确保字体名字没有错误。
2. `typst fonts --font-path path/to/your-fonts` 指定字体目录。
3. 检查中文字体是否已经完全安装。


### 如何为设置各行段落的缩进？

使用 `#set par(first-line-indent: 2em)`：

```example
#set par(first-line-indent: 2em)

= 一级标题

豫章故郡，洪都新府。星分翼轸，地接衡庐。襟三江而带五湖，控蛮荆而引瓯越。物华天宝，龙光射牛斗之墟；人杰地灵，徐孺下陈蕃之榻。雄州雾列，俊采星驰。

台隍枕夷夏之交，宾主尽东南之美。都督阎公之雅望，棨戟遥临；宇文新州之懿范，襜帷暂驻。

十旬休假，胜友如云；千里逢迎，高朋满座。腾蛟起凤，孟学士之词宗；紫电青霜，王将军之武库。家君作宰，路出名区；童子何知，躬逢胜饯。

== 二级标题

时维九月，序属三秋。潦水尽而寒潭清，烟光凝而暮山紫。俨骖騑于上路，访风景于崇阿。临帝子之长洲，得天人之旧馆。

层峦耸翠，上出重霄；飞阁流丹，下临无地。鹤汀凫渚，穷岛屿之萦回；桂殿兰宫，即冈峦之体势。
```

缺点是标题下的第一行没有缩进。为了解决这个问题，我们有两种办法：

**第一种办法：手动加入缩进。**

```example
#set par(first-line-indent: 2em)

#let indent = h(2em)

= 一级标题

#indent 豫章故郡，洪都新府。星分翼轸，地接衡庐。襟三江而带五湖，控蛮荆而引瓯越。物华天宝，龙光射牛斗之墟；人杰地灵，徐孺下陈蕃之榻。雄州雾列，俊采星驰。

台隍枕夷夏之交，宾主尽东南之美。都督阎公之雅望，棨戟遥临；宇文新州之懿范，襜帷暂驻。

十旬休假，胜友如云；千里逢迎，高朋满座。腾蛟起凤，孟学士之词宗；紫电青霜，王将军之武库。家君作宰，路出名区；童子何知，躬逢胜饯。

== 二级标题

#indent 时维九月，序属三秋。潦水尽而寒潭清，烟光凝而暮山紫。俨骖騑于上路，访风景于崇阿。临帝子之长洲，得天人之旧馆。

层峦耸翠，上出重霄；飞阁流丹，下临无地。鹤汀凫渚，穷岛屿之萦回；桂殿兰宫，即冈峦之体势。
```

这样做的优点是可以手动控制缩进，缺点是手动缩进不太方便。

**第二种办法：使用假段落自动加入缩进。**

```example
#set par(first-line-indent: 2em)

#let fake_par = {
  v(-1em)
  box()
}

#show heading: it => {
  it
  fake_par
}

= 一级标题

豫章故郡，洪都新府。星分翼轸，地接衡庐。襟三江而带五湖，控蛮荆而引瓯越。物华天宝，龙光射牛斗之墟；人杰地灵，徐孺下陈蕃之榻。雄州雾列，俊采星驰。

台隍枕夷夏之交，宾主尽东南之美。都督阎公之雅望，棨戟遥临；宇文新州之懿范，襜帷暂驻。

十旬休假，胜友如云；千里逢迎，高朋满座。腾蛟起凤，孟学士之词宗；紫电青霜，王将军之武库。家君作宰，路出名区；童子何知，躬逢胜饯。

== 二级标题

时维九月，序属三秋。潦水尽而寒潭清，烟光凝而暮山紫。俨骖騑于上路，访风景于崇阿。临帝子之长洲，得天人之旧馆。

层峦耸翠，上出重霄；飞阁流丹，下临无地。鹤汀凫渚，穷岛屿之萦回；桂殿兰宫，即冈峦之体势。
```

这样做的优点是可以自动首行缩进，缺点是其中的 `v(-1em)` 会造成标题和首行段落的间距出现问题。


### 如何让行内数学公式显示为行间数学公式的大小？

可以通过 `display()` 函数实现。

```example
行内数学公式（脚本模式） $integral x dif x$

行内数学公式（展示模式） $display(integral x dif x)$
```

注意，由于 `display` 也是一个函数，所以在其内部的逗号 `,` 要进行转义 `\,`。

每次都要手动打 `display` 感觉很麻烦，能不能默认自动加上呢？

其实是有办法实现的，如下所示，借助 `label`、`box`、`show` 和块状公式，就可以实现了。

```example
#show math.equation.where(block: false): it => [#math.equation(block: true, numbering: none, it)<inline-math-equation>]
#show <inline-math-equation>: it => box(it)

行内数学公式（自动变为展示模式） $integral x dif x$
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
