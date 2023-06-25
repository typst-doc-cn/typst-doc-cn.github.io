# Typst 中文文档

社区驱动的非官方 Typst 中文文档.

https://typst-doc-cn.github.io/docs/


## 贡献

1. Fork 仓库
2. 更改 `./docs/src` 目录下的 Markdown 文件 (不用修改文件名标题)
3. 如果在 `./docs/src` 目录下不存在对应的 Markdown 文件，则用全局搜索的方式搜索对应的内容（一般是一个注释），然后进行编辑
发起一个 Pull Request
4. 如果需要的话，也可以在文档的末尾处留下翻译者的名字

PS: Reference 中的 *CONTENT* 和 *COMPUTE* 部分需要深入到 `./library/src/text/misc.rs` 这类代码文件中的注释中修改.

如果想要贡献的话, 可以先使用微信号 orangex4 联系我, 我们会使用微信群和石墨文档进行协作.

当然也可以直接尝试翻译和发起 Pull Request.


## 技术细节

Typst 的文档生成方式比较复杂, 其与 Typst 的代码文件紧密耦合, 例如示例部分的图片都是编译时生成的, 如果不清楚其中的细节, 请谨慎修改.


## 本地生成

本地生成是非必须的, 但是它很适合你在本地查看生成的网页是否正确.

首先你需要 clone 本仓库, 并安装 `cargo` 工具链, 以及 Python 和 Python 包 `jinja2`.

```sh
cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture
python ./gen.py
```

最后编译得到的文件位于 `./dist`.