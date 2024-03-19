---
description: |
  中文用户指南
---

# 中文用户指南
本页面并不属于官方文档的内容，而是 Typst 中文社区所撰写的面向中文用户的指南。
如果您对在 Typst 中使用中文有什么使用心得，也欢迎在参与贡献！

## Typst 的优势

Typst 是可用于出版的可编程标记语言，拥有变量、函数与包管理等现代编程语言的特性，注重于科学写作 (science writing)，定位与 LaTeX 相似。

- **语法简洁**：上手难度跟 Markdown 相当，文本源码阅读性高，不会像 LaTeX 一样充斥着反斜杠与花括号。
- **编译速度快**：Typst 使用 Rust 语言编写，即 typ(e+ru)st，目标运行平台是WASM，即浏览器本地离线运行；也可以编译成命令行工具，采用一种增量编译算法和一种有约束的版面缓存方案，文档长度基本不会影响编译速度，且编译速度与常见 Markdown 渲染引擎渲染速度相当。
- **环境搭建简单**：不需要像 LaTeX 一样折腾几个 G 的开发环境，原生支持中日韩等非拉丁语言，无论是官方 Web App 在线编辑，还是使用 VS Code 安装插件本地开发，都是即开即用。
- **现代编程语言**：Typst 是可用于出版的可编程标记语言，拥有变量、函数、包管理与错误检查等现代编程语言的特性，同时也提供了闭包等特性，便于进行函数式编程。以及包括了 [标记模式]、{脚本模式} 与 $数学模式$ 等多种模式的作用域，并且它们可以不限深度地、交互地嵌套。并且通过 [包管理](https://typst-doc-cn.github.io/docs/packages/)，你不再需要像 TexLive 一样在本地安装一大堆并不必要的宏包，而是按需自动从云端下载。


## 目前仍存在的 CJK 问题 { #question }

参考 [Discord](https://discord.com/channels/1054443721975922748/1176062736514429008) 的记录，可知目前仍存在：

- 行内代码或行内数学公式与中文之间的自动空格 [#2702](https://github.com/typst/typst/issues/2702) [#2703](https://github.com/typst/typst/issues/2703)。
- 变体字体、伪粗体、斜体等字体支持 [#725](https://github.com/typst/typst/issues/725)。
- 不能简单地实现首段缩进 [#311](https://github.com/typst/typst/issues/311)。
- 暂时无法忽略 CJK 字符之间的单个换行符自动转换成的空格 [#792](https://github.com/typst/typst/issues/792)。
- 有时候段落开始的 CJK 标点符号没有被调整 [#2348](https://github.com/typst/typst/issues/2348)。


## 常见 Q&A { #question-and-answer }

### 如何进一步进阶？

除了参考，还可以考虑阅读 [typst-examples-book](https://sitandr.github.io/typst-examples-book/book/)，里面包含了一些 Typst 的高级知识、简单示例，以及一些最佳实践。

例如简单地实现类似 Markdown 中的引用文本样式：

```example
+ #lorem(10) \
  #rect(fill: luma(240), stroke: (left: 0.25em))[
    *Solution:* #lorem(10)

    $ a_(n+1)x^n = 2... $
  ]
```

### 如何使用 VS Code 进行本地编辑？

1. 在 [VS Code](https://code.visualstudio.com/) 中打开任意工作目录。
2. 在 VS Code 中安装 [Typst LSP](https://marketplace.visualstudio.com/items?itemName=nvarner.typst-lsp) 和 [Typst Preview](https://marketplace.visualstudio.com/items?itemName=mgt19937.typst-preview) 插件。前者负责语法高亮和错误检查，后者负责预览。
    - 也推荐下载 [Typst Companion](https://marketplace.visualstudio.com/items?itemName=CalebFiggers.typst-companion) 插件，其提供了例如 `Ctrl + B` 进行加粗等便捷的快捷键。
    - 还可以下载 [Typst Sync](https://marketplace.visualstudio.com/items?itemName=OrangeX4.vscode-typst-sync) 和 [Typst Sympy Calculator](https://marketplace.visualstudio.com/items?itemName=OrangeX4.vscode-typst-sympy-calculator) 插件，前者提供了本地包的云同步功能，后者提供了基于 Typst 语法的科学计算器功能。
3. 新建一个 `test.typ` 文件，写入内容 `# Hello World`。
4. 按下 `Ctrl + K V`，即可同步增量渲染与预览，还提供了光标双向定位功能。


### 如何为中英文设置不同的字体？

可以使用 text 里面的 fallback 特性。
Typst 中的 font 参数可以接收一个数组，会根据字体里有无当前字符来依次选择字体。
因此我们只需要传入一个英文字体后接中文字体的数组，就可以达到为中英文设置不同的字体的效果。

```example
Hello World 你好世界

#[
  #set text(font: ("IBM Plex Serif", "Noto Sans CJK SC"), lang: "zh", region: "cn")

  Hello World 你好世界
]
```

如果你还需要对中文字体进行特殊处理，例如只缩小中文字体的大小，可以考虑用正则表达式进行 hack：

```example
#show regex("\p{sc=Hani}+"): set text(size: 0.8em)

Hello World 你好世界
```


### 为什么我设置的字体没有生效？

如果中文字体不符合 typst 要求，那么它不会选择你声明的字体，例如字体的变体数量不够，参考更详细的 [issue](https://github.com/typst/typst/issues/725)。

1. `typst fonts` 查看系统字体，确保字体名字没有错误。
2. `typst fonts --font-path path/to/your-fonts` 指定字体目录。
3. `typst fonts --variants` 查看字体变体。
4. 检查中文字体是否已经完全安装。


### 为什么连续标点会挤压在一起？

如果字体与 `text(lang: .., region: ..)` 不匹配，可能会导致连续标点的挤压。例如字体不是中国大陆的，标点压缩会出错；反之亦然。


### 如何添加中文斜体？

中文斜体一般使用楷体替代，你可以 [通过 show-set 规则实现](https://github.com/typst/typst/issues/725)：

```example
#show emph: text.with(font: ("Linux Libertine", "STKaiti"))

孔乙己#emph[上大人]

A quick _brown_
```

如果你真的需要伪斜体，可以考虑使用 [@Enivex](https://github.com/Enivex) 在 [Discord](https://discord.com/channels/1054443721975922748/1054443722592497796/1175967383630921848) 给出的一段 hack 代码：

```example
#let skew(angle, vscale: 1, body) = {
  let (a, b, c, d) = (1, vscale * calc.tan(angle), 0, vscale)
  let E = (a + d) / 2
  let F = (a - d) / 2
  let G = (b + c) / 2
  let H = (c - b) / 2
  let Q = calc.sqrt(E * E + H * H)
  let R = calc.sqrt(F * F + G * G)
  let sx = Q + R
  let sy = Q - R
  let a1 = calc.atan2(F, G)
  let a2 = calc.atan2(E, H)
  let theta = (a2 - a1) / 2
  let phi = (a2 + a1) / 2
  
  set rotate(origin: bottom + center)
  set scale(origin: bottom + center)
  
  rotate(phi, scale(x: sx * 100%, y: sy * 100%, rotate(theta, body)))
}

#let fake-italic(body) = skew(-12deg, body)

#let shadowed(body) = box(place(skew(-50deg, vscale: 0.8, text(fill: luma(200), body))) + place(body))

#fake-italic[中文伪斜体]

#shadowed[还可以用来实现文字阴影效果]
```


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

每次都要手动打 `display` 感觉很麻烦，能不能默认自动加上呢？如下所示，借助 `label` 和 `show` 即可实现。

```example
#show math.equation.where(block: false): it => {
  if it.has("label") and it.label == label("displayed-inline-math-equation") {
    it
  } else {
    [$display(it)$<displayed-inline-math-equation>]
  }
}

行内数学公式（展示模式） $display(integral x dif x)$
```


### 如何嵌入 PDF 文件？

你暂时没有办法在 Typst 里嵌入 PDF 文件，但是你可以先使用 [在线工具](https://cloudconvert.com/pdf-to-svg) 将 PDF 文件转换为 SVG 文件，然后嵌入 svg 文件。


### 如何根据章节对图表和公式进行编码？

可以使用 [i-figured](https://github.com/RubixDev/typst-i-figured) 包。


### 如何编写复杂表格或编写简洁的表格？

复杂表格：可以使用 [tablex](https://github.com/PgBiel/typst-tablex) 包。

类 Markdown 表格：可以使用 [tablem](https://github.com/OrangeX4/typst-tablem) 包。


## 一些 Typst 中文资源列表 { #resources }

可以查看 [Awesome Typst 中文版](https://github.com/typst-doc-cn/awesome-typst-cn) 中文版，以及浏览 [第三方包](https://typst-doc-cn.github.io/docs/packages/)。

**中国大学论文**：

- [pkuthss-typst](https://github.com/lucifer1004/pkuthss-typst): 北京大学学位论文模板
- [BUAA-typst](https://github.com/cherichy/BUAA-typst): 北京航空航天大学学位论文模板
- [bupt-typst](https://github.com/QQKdeGit/bupt-typst): 北京邮电大学本科学士学位论文模板
- [HUST-typst-template](https://github.com/werifu/HUST-typst-template): 用于华科毕业设计（本科）的 typst 模板。
- [SHU-Bachelor-Thesis-Typst](https://github.com/shuosc/SHU-Bachelor-Thesis-Typst): 上海大学本科毕业论文 typst 模板 (开发ing)
- [sysu-thesis-typst](https://github.com/howardlau1999/sysu-thesis-typst): 中山大学学位论文 Typst 模板
- [ZJGSU-typst-template](https://github.com/jujimeizuo/ZJGSU-typst-template): 浙江工商大学毕业设计（本科）的 typst 模板。
- [CQUPTypst](https://github.com/jerrita/CQUPTypst): 一个 Typest 模板，但是大专 
- [zjut-report-typst](https://github.com/zjutjh/zjut-report-typst): 浙江工业大学一些实验报告的 Typst 模板
- [HIT-Thesis-Typst](https://github.com/chosertech/HIT-Thesis-Typst): 适用于哈尔滨工业大学学位论文的 Typst 模板
- [nju-thesis-typst](https://github.com/nju-lug/nju-thesis-typst): 南京大学学位论文 Typst 模板，使用 Typst 包管理、闭包等现代编程语言特性开发，一个更方便编辑和拓展的模板
- [nuist-thesis-typst](https://github.com/Dustella/nuist-thesis-typst): 南京信息工程大学本科生毕业论文/设计 Typst 模板，分叉自 nju-thesis-typst

**中文简历**：

- [uniquecv-typst](https://github.com/gaoachao/uniquecv-typst): 一个使用 Typst 编写的简历模板，基于 uniquecv
- [typst-cv-miku](https://github.com/ice-kylin/typst-cv-miku): 简历模板，有多种版本，包括中文
- [awesomeCV-Typst](https://github.com/mintyfrankie/awesomeCV-Typst): 一份参考 `Awesome-CV` 的简历模版，支持多语言简历管理
- [Chinese-Resume-in-Typst](https://github.com/OrangeX4/Chinese-Resume-in-Typst): 使用 Typst 编写的中文简历, 语法简洁, 样式美观, 开箱即用, 可选是否显示照片
- [neet-cv](https://github.com/kznr02/neet-cv): 作者自用后开源的一份使用 typst 自制的中文简历模板，具有简单的使用方法，其中有部分参考 `wondercv`，开箱即用，简洁美观

**幻灯片**：

- [touying](https://github.com/touying-typ/touying) - 拥有强大功能和丰富模板的幻灯片包，包括详细的[中文文档](https://touying-typ.github.io/touying/zh/docs/intro/)
- [polylux](https://github.com/andreasKroepelin/polylux) - 在 Typst 中创建演示幻灯片包
- [pinit](https://github.com/OrangeX4/typst-pinit) - 并非幻灯片包，而是一个好用的相对定位工具包
