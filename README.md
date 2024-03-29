# hello-rust

[Rust语言圣经(Rust Course)](https://course.rs/)

# Mac 环境下 rust的安装

在mac 环境下比较便捷的安装方式主要有两种，一种是直接通过 `brew install rust` ，另一种方式是通过安装 rustup 来间接安装 rust ，这里推荐使用 安装rustup的方式 来间接管理 rust环境。

## rustup 介绍
[rustup包管理工具文档](https://www.bookstack.cn/read/rust-edition-guide-cn/src-rust-2018-rustup-for-managing-rust-versions.md)

## 安装rustup-init

1. 首先通过 brew 安装 rustup-init
   `brew install rustup-init`
   如果你已经通过 `brew install rust` 的方式 安装过rust了，请先执行`brew uninstall rust` 卸载之前安装的rust

2. 执行 rustup-init 安装 rust相关环境
   `rustup-init`

3. 安装 toolchain , 这里可以直接安装当前系统环境对应的stable版本
   `rustup toolchain install stable`

4. 执行`rustup show` 可以查看当前安装的rust 相关环境

   `rustup show`
   比如，我的macbook-pro M1 会输出

   ```bash
   Default host: aarch64-apple-darwin
   rustup home:  /Users/admin/.rustup
   
   stable-aarch64-apple-darwin (default)
   rustc 1.62.0 (a8314ef7d 2022-06-27)
   ```

5. 安装 Rust 的其他版本，执行 `rustup install`：
   ```
   $ rustup install 1.30.0
   ```
   三种最新的版本:
   ```
   $ rustup install stable
   $ rustup install beta
   $ rustup install nightly
   ```

## rust 镜像源配置

字节跳动 Rust 镜像源以及安装rust

地址：https://rsproxy.cn/

1. 字节跳动 `crates.io` 镜像源
   `vim ~/.cargo/config`

```toml
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

2. Rustup 镜像

   `~/.zshrc` 或者` ~/.bashrc`

`vim ~/.bashrc`

```shell
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

`source ~/.bashrc`

3. 安装Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh      
```


## rustup 管理
### 升级

升级所有安装的版本，你可以执行：

```
$ rustup update
```

这将查看您已安装的所有内容，如果有新版本，将会更新。

### 版本管理

设置非 `stable` 的为默认版本：

```
$ rustup toolchain default nightly
```

使用一个 toolchain 而不是默认的，`rustup run`：

```
$ rustup run nightly cargo build
```

还有一个别名，这个更短一些：

```
$ cargo +nightly build
```

如果您希望每个目录具有不同的默认值，那也很容易！ 如果你在项目中运行它：

```
$ rustup override set nightly
```

然后当你在那个目录中时，`rustc` 或 `cargo` 的任何调用都将使用该工具链。 要与其他人共享，可以使用工具链的内容创建一个 `rust-toolchain` 文件，并将其检入源代码管理中。 现在，当有人克隆您的项目时，他们将获得正确的版本，而无需自己“覆盖集合”。

### 安装其他目标 (target)

Rust 支持交叉编译到其他平台，Rustup 可以帮助您管理它们。 例如，要使用 MUSL：

```
$ rustup target add x86_64-unknown-linux-musl
```

然后，你可以：

```
$ cargo build --target=x86_64-unknown-linux-musl
```

查看所有安装的目标：

```
$ rustup target list
```

### 安装组件

组件用于安装某些类型的工具。虽然大多数工具都提供了“cargo-install”，但有些工具需要深入集成到编译器中。 Rustup 确切地知道您正在使用的编译器版本，因此它只具有这些工具所需的信息。

组件是每个工具链，因此如果您希望它们可用于多个工具链，则需要多次安装它们。 在下面的示例中，添加一个 `--toolchain` 标志，设置为您要安装的工具链，例如 `nightly`。 如果没有此标志，它将安装默认工具链的组件。

要查看可以安装的完整组件列表：

```
$ rustup component list
```

接下来，让我们谈谈一些流行的组件以及何时需要安装它们。

#### `rust-docs`, 本地文档

安装工具链时，默认情况下会安装此第一个组件。 它包含 Rust 的文档副本，以便您可以脱机阅读。

此组件暂时无法删除; 如果感兴趣，请对 [this issue](https://github.com/rust-lang-nursery/rustup.rs/issues/998) 发表评论。

#### `rust-src` 标准库代码的拷贝

`rust-src` 组件可以为您提供 Rust 的源代码的本地副本。你为什么需要这个？好吧，像 Racer 这样的自动完成工具使用这些信息来了解你要调用的函数的更多信息。

```
$ rustup component add rust-src
```

#### “预览”组件

“预览”阶段有几个组件。 这些组件的名称目前都有 `-preview`，这表明它们还没有100％准备好进行一般使用。 请尝试一下并给我们反馈，但要知道他们不遵循 Rust 的稳定性保证，并且仍然在积极地改变，可能是以向后不兼容的方式。

##### `rustfmt-preview` 自动代码格式化

![Minimum Rust version: 1.24](https://static.sitestack.cn/projects/rust-edition-guide-cn/f72856ae70f49dc23b02d263f0b1b6da.svg)

如果您希望自动格式化代码，可以安装此组件：

```
$ rustup component add rustfmt-preview
```

这将安装两个工具，`rustfmt` 和 `cargo-fmt`，它们将为您自动格式化代码！ 例如：

```
$ cargo fmt
```

将重新格式化您的整个 cargo 项目。

##### `rls-preview` 为了 IDE 集成

![Minimum Rust version: 1.21](https://static.sitestack.cn/projects/rust-edition-guide-cn/73e91561a5b71e06ecc2ec5a8c20ea1f.svg)

许多 IDE 功能都是基于 [`langserver`协议](http://langserver.org/) 构建的。要使用这些 IDE 获得对 Rust 的支持，您需要安装 Rust 语言服务器，即“RLS”：

```
$ rustup component add rls-preview
```

你的 IDE 应该从那拿到它。

##### `clippy-preview` 更多的 lints

要获得更多的 lints 来帮助你编写 Rust 代码，你可以安装 `clippy`：

```
$ rustup component add clippy-preview
```

This will install `cargo-clippy` for you:

```
$ cargo clippy
```

更多信息，查阅 [clippy’s documentation](https://github.com/rust-lang-nursery/rust-clippy).

##### `llvm-tools-preview` 使用额外的LLVM工具

如果您想使用 `lld` 链接器或其他工具，如 `llvm-objdump` 或 `llvm-objcopy`，您可以安装此组件：

```
$ rustup component add llvm-tools-preview
```

这是最新的组件，因此目前没有良好的文档。
