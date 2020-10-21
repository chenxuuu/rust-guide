## 配置 Cargo 国内镜像源

crates.io 是 Rust 官方的 crate 管理仓库，放置在 github 仓库。Cargo 的“注册表源”与 crates.io 本身相同，也就是说，Cargo 也有一个在 github 存储库中提供的索引。该存储库匹配 `crates.io index` 的格式，即 github 仓库 `https://github.com/rust-lang/crates.io-index`，由该存储库的索引指示下载包的配置。

对于中国大陆用户，因为网络原因，推荐使用国内镜像源。目前国内 cargo 镜像源有：清华大学源、中国科学技术大学源、上海交通大学源，以及 rustcc 社区源。

Cargo 支持**用另一个来源更换一个来源**的能力，通过 `$HOME/.cargo/config` 文件配置。自定义 cargo 源有两种方法，推荐使用第一种：

1. 创建 `$HOME/.cargo/config` 文件（各操作系统及版本均大致相同），然后在 config 文件内写入下述配置内容。其中协议推荐使用 git，但对于 https 和 git 协议，一般各镜像源都支持，并且是可以互换的。如果你所处的环境中不允许使用 git 协议，或者配置 git 协议后不能正常获取和编译 crate，可以换 https 协议再试试。

``` toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```

2. 或者在项目工程结构中，与 Cargo.toml 同级目录的 .cargo 文件夹下创建 config 文件，config 文件配置方法和内容与第一种相同。
