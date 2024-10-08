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
- **编译速度快**：Typst 使用 Rust 语言编写，即 typ(esetting+ru)st，目标运行平台是WASM，即浏览器本地离线运行；也可以编译成命令行工具，采用一种增量编译算法和一种有约束的版面缓存方案，文档长度基本不会影响编译速度，且编译速度与常见 Markdown 渲染引擎渲染速度相当。
- **环境搭建简单**：不需要像 LaTeX 一样折腾几个 G 的开发环境，原生支持中日韩等非拉丁语言，无论是官方 Web App 在线编辑，还是使用 VS Code 安装插件本地开发，都是即开即用。
- **现代编程语言**：Typst 是可用于出版的可编程标记语言，拥有变量、函数、包管理与错误检查等现代编程语言的特性，同时也提供了闭包等特性，便于进行函数式编程。以及包括了 [标记模式]、{脚本模式} 与 $数学模式$ 等多种模式的作用域，并且它们可以不限深度地、交互地嵌套。并且通过 [包管理](https://typst-doc-cn.github.io/docs/packages/)，你不再需要像 TexLive 一样在本地安装一大堆并不必要的宏包，而是按需自动从云端下载。


## 目前仍存在的 CJK 问题 { #question }

参考 [Discord](https://discord.com/channels/1054443721975922748/1176062736514429008) 的记录，可知目前仍存在：

- 行内代码或行内数学公式与中文之间的自动空格 [#2702](https://github.com/typst/typst/issues/2702) [#2703](https://github.com/typst/typst/issues/2703)。
- 不能简单地实现首段缩进 [#311](https://github.com/typst/typst/issues/311)。
- 暂时无法忽略 CJK 字符之间的单个换行符自动转换成的空格 [#792](https://github.com/typst/typst/issues/792)。
- 有时候段落开始的 CJK 标点符号没有被调整 [#2348](https://github.com/typst/typst/issues/2348)。


## 常见 Q&A { #question-and-answer }

### 如何进一步进阶？

除了参考，还可以考虑阅读 [typst-examples-book](https://sitandr.github.io/typst-examples-book/book/)，里面包含了一些 Typst 的高级知识、简单示例，以及一些最佳实践。

---

**请阅读 [小蓝书](https://typst-doc-cn.github.io/tutorial/) 和 [Typst 中文社区导航 FAQ](https://typst-doc-cn.github.io/guide/)。**

**请阅读 [小蓝书](https://typst-doc-cn.github.io/tutorial/) 和 [Typst 中文社区导航 FAQ](https://typst-doc-cn.github.io/guide/)。**

**请阅读 [小蓝书](https://typst-doc-cn.github.io/tutorial/) 和 [Typst 中文社区导航 FAQ](https://typst-doc-cn.github.io/guide/)。**

---

### 如何使用 VS Code 进行本地编辑？

1. 在 [VS Code](https://code.visualstudio.com/) 中打开任意工作目录。
2. 在 VS Code 中安装 [Tinymist Typst](https://marketplace.visualstudio.com/items?itemName=myriad-dreamin.tinymist) 插件，其提供了语法高亮、错误检查和预览等功能。**不要安装 Typst LSP 插件，该插件已过时。**
    - 也推荐下载 [Typst Companion](https://marketplace.visualstudio.com/items?itemName=CalebFiggers.typst-companion) 插件，其提供了例如 `Ctrl + B` 进行加粗等便捷的快捷键。
    - 你还可以下载我开发的 [Typst Sync](https://marketplace.visualstudio.com/items?itemName=OrangeX4.vscode-typst-sync) 和 [Typst Sympy Calculator](https://marketplace.visualstudio.com/items?itemName=OrangeX4.vscode-typst-sympy-calculator) 插件，前者提供了本地包的云同步功能，后者提供了基于 Typst 语法的科学计算器功能。
3. 新建一个 `test.typ` 文件，写入内容 `= Hello World`。
4. 按下 `Ctrl + K V`，即可同步增量渲染与预览。


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


### 如何添加中文粗体和斜体？

可以使用 [cuti](https://github.com/csimide/cuti) 包。


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

#let fakepar = context {
  box()
  v(-measure(block() + block()).height)
}

#show heading: it => {
  it
  fakepar
}

= 一级标题

豫章故郡，洪都新府。星分翼轸，地接衡庐。襟三江而带五湖，控蛮荆而引瓯越。物华天宝，龙光射牛斗之墟；人杰地灵，徐孺下陈蕃之榻。雄州雾列，俊采星驰。

台隍枕夷夏之交，宾主尽东南之美。都督阎公之雅望，棨戟遥临；宇文新州之懿范，襜帷暂驻。

十旬休假，胜友如云；千里逢迎，高朋满座。腾蛟起凤，孟学士之词宗；紫电青霜，王将军之武库。家君作宰，路出名区；童子何知，躬逢胜饯。

== 二级标题

时维九月，序属三秋。潦水尽而寒潭清，烟光凝而暮山紫。俨骖騑于上路，访风景于崇阿。临帝子之长洲，得天人之旧馆。

层峦耸翠，上出重霄；飞阁流丹，下临无地。鹤汀凫渚，穷岛屿之萦回；桂殿兰宫，即冈峦之体势。
```

PS: 例子来源于 [Myriad-Dreamin](https://github.com/Myriad-Dreamin)


### 如何嵌入 PDF 文件？

你暂时没有办法在 Typst 里嵌入 PDF 文件，但是你可以先使用 [在线工具](https://cloudconvert.com/pdf-to-svg) 将 PDF 文件转换为 SVG 文件，然后嵌入 svg 文件。


### 如何根据章节对图表和公式进行编码？

可以使用 [i-figured](https://github.com/RubixDev/typst-i-figured) 包。


### 如何编写复杂表格或编写简洁的表格？

复杂表格：可以查看 [Table Guide](https://typst.app/docs/guides/table-guide/)。

类 Markdown 表格：可以使用 [tablem](https://github.com/OrangeX4/typst-tablem) 包。

### 如何更换不同的参考文献格式？

Typst (>=0.10.0) 可以使用 csl 文件指定参考文献格式，见 [`bibliography` 的文档](https://typst.app/docs/reference/model/bibliography/#parameters-style)。

Typst 内置了中文常用的 GB/T-7714-2015 格式，其他 GB/T-7714-2015 变体可以查看 [GB/T 7714—2015 相关的 CSL 样式](https://github.com/redleafnew/Chinese-STD-GB-T-7714-related-csl)。

### 为什么指定参考文献 csl 后，报错 `failed to load CSL style`？

**报错1：** ``(duplicate field `layout`)``

Typst 暂不支持 CSL-M 标准，可以注释掉多余的 `<layout>` **临时**解决。

在 csl 文件里搜索 `bibliography`，这里通常有多个 `<layout>` ，一般建议注释掉 `<layout locale="en">` 这一段 `<layout>` 。例子如下

```xml
<bibliography entry-spacing="0" et-al-min="4" et-al-use-first="3" second-field-align="flush">
  <!--
  <layout locale="en">
    <text variable="citation-number" prefix="[" suffix="]"/>
    <text macro="entry-layout"/>
  </layout>
  -->
  <layout>
    <text variable="citation-number" prefix="[" suffix="]"/>
    <text macro="entry-layout"/>
  </layout>
</bibliography>
```

（示例来自 http://www.zotero.org/styles/china-national-standard-gb-t-7714-2015-numeric ，原作者见此文件，依 CC-BY-SA 3.0 协议使用）

这样修改之后，csl 根据文献语言自动使用“等”或“et al.”的功能会失效，请见下一条 Q&A 的问题1。

**报错2：** ``(unknown variant `institution`, expected one of `name`, `et-al`, `label`, `substitute`)`` 

在 csl 文件里注释掉不支持的部分。

### 为什么参考文献格式与预期不符？

**问题1：** 希望在中文文献使用 `等`、在西文文献使用 `et al.`，但 Typst 均显示为 `等`（或均显示为 `et al.`）。

Typst 暂不支持 CSL-M 标准，因此暂时无法通过修改 csl 文件实现中文西文自动使用不同的文字（[typst/typst#2793](https://github.com/typst/typst/issues/2793), [typst/citationberg#5](https://github.com/typst/citationberg/issues/5), [typst/hayagriva#126](https://github.com/typst/hayagriva/pull/126)）。

如果参考文献均为同一种语言，可以为参考文献部分设定语言，如：

```example
#set text(lang: "zh")
一些内容 @tbs1 。

#heading(level: 1, numbering: none)[参考文献]
#{
  set text(lang: "en")
  bibliography(
    "many-authors.bib",
    style: "gb-7714-2015-numeric",
    title: none
  )
}
```

如果需要实现中文西文自动使用不同的文字，可以使用正则替换魔法，请见 [nju-lug/modern-nju-thesis#3](https://github.com/nju-lug/modern-nju-thesis/issues/3)。

**问题2：** (numeric 格式) 连续引用多条文献时，应当折叠为 `[1-4]` ，但是 Typst 折叠为 `[1,4]` 。

hayagriva 已知 bug [typst/hayagriva#154](https://github.com/typst/hayagriva/issues/154)。

可以通过将 csl 文件里的 `after-collapse-delimiter=","` 改成 `after-collapse-delimiter="-"` 临时解决。请注意，这样做并不符合 CSL 规范，修改后的文件不应当用于 Zotero 等文献管理软件。**待 hayagriva 修复此 bug 后，需要改回**。

**问题3：** 引文条目中 `. ` 部分丢失。

在 csl 中修改生成引文条目的 `macro`，向缺少 `. ` 的部分添加 `<group delimiter=". ">`。

**问题4：** 参考文献不显示 bib 中的 `note` 。

目前暂不支持（[typst/hayagriva#91](https://github.com/typst/hayagriva/issues/91)）。

**问题5：** 学位论文 `[D]` 后不显示 `地点: 学校名称, 年份.` 。

Typst 暂不支持 `school` `institution` 作为 `publisher` 的别名，亦不支持解析 csl 中的 `institution`（[typst/hayagriva#112](https://github.com/typst/hayagriva/issues/112)）。如需修复，请手动修改 bib 文件内对应条目，在 `school = {学校名称},` 下加一行 `publisher = {学校名称},` ，如：

```
@phdthesis{alterego,
  type = {{超高校级学位论文}},
  title = {{基于图书室的笔记本电脑的 Alter Ego 系统}},
  author = {不二咲, 千尋},
  year = {2010},
  address = {某地},
  school = {私立希望ヶ峰学園},
  publisher = {私立希望ヶ峰学園},
}
```

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
- [SEU-Typst-Template](https://github.com/csimide/SEU-Typst-Template): 东南大学本科毕业设计与学位论文模板
- [HZAU_Typst](https://github.com/wagaaa/HZAU_Typst?tab=readme-ov-file): 华中农业大学本科生/研究生学位论文模板（非官方）

**中文简历**：

- [uniquecv-typst](https://github.com/gaoachao/uniquecv-typst): 一个使用 Typst 编写的简历模板，基于 uniquecv
- [typst-cv-miku](https://github.com/ice-kylin/typst-cv-miku): 简历模板，有多种版本，包括中文
- [awesomeCV-Typst](https://github.com/mintyfrankie/awesomeCV-Typst): 一份参考 `Awesome-CV` 的简历模版，支持多语言简历管理
- [Chinese-Resume-in-Typst](https://github.com/OrangeX4/Chinese-Resume-in-Typst): 使用 Typst 编写的中文简历, 语法简洁, 样式美观, 开箱即用, 可选是否显示照片
- [neet-cv](https://github.com/kznr02/neet-cv): 作者自用后开源的一份使用 typst 自制的中文简历模板，具有简单的使用方法，其中有部分参考 `wondercv`，开箱即用，简洁美观

**幻灯片**：

- [touying](https://github.com/touying-typ/touying) - 拥有强大功能和丰富模板的幻灯片包，包括详细的[中文文档](https://touying-typ.github.io/touying/zh/docs/intro/)
- [pinit](https://github.com/OrangeX4/typst-pinit) - 并非幻灯片包，而是一个好用的相对定位工具包
