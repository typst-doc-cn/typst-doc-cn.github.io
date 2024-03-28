# Typst 中文文档网站

社区驱动的非官方 Typst 中文文档.

https://typst-doc-cn.github.io/docs/

GitHub Repo：https://github.com/typst-doc-cn/typst-doc-cn.github.io

Gitee 镜像：https://gitee.com/orangex4/typst-doc-cn.github.io

## 贡献

1. Fork 仓库
2. 更改 `./docs/src` 目录下的 Markdown 文件
3. 更改 `./docs/i18n` 目录下的 Yaml 文件
4. 如果你想为「第三方包」（packages）页面的包添加对应翻译，可以修改 `./static/assets/index2cn.json` 文件
5. 遵守翻译规范：
  1. 中英文之间添加一个空格
  2. 尽量使用中文标点符号
  3. 不确定的术语可以参考术语表或者其他页面的翻译
  4. 示例 (example) 里的英文不需要翻译成中文
6. 发起一个 Pull Request
7. 如果需要的话，也可以在文档的末尾处留下翻译者的名字

如果想要贡献, 并且有任何问题的话, 可以加入 QQ 群 793548390 联系 OrangeX4.

当然也可以直接尝试翻译和发起 Pull Request.


## 技术细节

Typst 的文档生成是与 Typst 源码紧耦合的, 具体体现在:

1. Typst 开发者使用 Rust 写了一个 `docs` 模块, 文档生成的大部分工作都在这里进行;
2. Typst 的文档是使用一种 Markdown 的方言以及 Yaml 文件生成的, `docs/src` 路径下的文件;
3. `docs` 模块里的函数被执行后, 会动态地生成示例图片, 就像你们示例代码对应的右边的示例图片, 均为编译时生成的;
4. 参考部分对应的大部分页面, 例如 [`lorem`](https://typst-doc-cn.github.io/docs/reference/text/lorem/) 函数, 对应的 Markdown 部分是写在 `lorem` 函数对应的 Rust 代码文件的注释里的, 有点像 `docstring`;
5. Typst Repo 里面只有 `docs` 模块的代码, 没有对应的文档网页生成的代码, 那部分还没有开源;

因此我做了以下事情以生成这个 Typst 中文文档网页:

1. 修改了 `docs` 模块的部分代码, 在 `assets` 路径下生成了一个所有文档对应的 `docs.json` 文件, 这个文件里面的结构相对复杂, 但是包含了生成一个文档网站的全部内容;
2. `docs.json` 里面有多种类型的结构:
    1. 最简单的是 `html` 类型, 对应概览和教程, 以及参考的 `LANGUAGE` 大部分;
    2. 还有 `type` 类型, 对应的就是类型对应的页面;
    3. 还有 `func` 类型, 对应的就是函数对应的页面, 里面内容繁杂, 东西很多;
    4. 其他还有三种类型 `category`, `funcs`, `symbol`;
3. 我依照官网的 HTML 和 CSS 文件, 使用 `jinja2` 的语法写了几个 HTML 模板, 放在 `templates` 路径下;
4. 然后我写了 `gen.py`, 用于将 `docs.json` 使用模板转换为最终的 Web 网站, 放在 `dist` 路径下;
5. 根据 `docs.json` 里的内容, 生成了 `i18n` 目录便于翻译;
6. 写了一个 GitHub Action 用于将生成 `dist` 部署到 GitHub Pages 上.

![](https://picgo-1258602555.cos.ap-nanjing.myqcloud.com/20230625213846.png)


## 本地生成

本地生成是非必须的, 但是它很适合你在本地查看生成的网页是否正确.

首先你需要 clone 本仓库, 并安装 `cargo` 工具链, 以及 Python 和 Python 包 `jinja2` 和 `pyyaml`.

```sh
# 修改了 `./docs/src` 目录则需要运行这两行命令
cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture
# 如果只是修改 `./docs/i18n` 目录则只需要运行这行命令
python ./gen.py
```

最后编译得到的文件位于 `./dist`.

如果你有安装 nodejs, 则你可以通过 `npx serve ./dist` 快速地在本地开启一个 web 静态服务器预览结果.
