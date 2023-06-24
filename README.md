# Typst 中文文档

社区驱动的非官方 Typst 中文文档.

https://typst-doc-cn.github.io/docs/


## 贡献

1. Fork 这个 Repo
2. 更改 `./docs` 目录下的 Markdown 文件 (由于存在限制, 不允许修改标题)
3. 发起一个 Pull Request

PS: Reference 中的 *CONTENT* 和 *COMPUTE* 部分需要深入到 `./library/src/text/misc.rs` 这类代码文件中的注释中修改.


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