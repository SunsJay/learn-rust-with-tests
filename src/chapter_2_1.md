# 安装Rust

Rust 语言官方安装指引可参考[这里](https://www.rust-lang.org/tools/install)

出于示范的目的，我们将展示在 OSX 系统中安装的步骤



## 安装

安装的过程非常容易，首先我们打开终端，输入 Rust 的一行安装命令（该命令对于类Unix的操作系统都是一样的），如果你是 Windows 的话，那需要在官网好好看下安装步骤，它不同与 Macos，Linux 之类的安装教程。

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

![image-20230327192725160](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230327192725160.png)

第一个就是默认安装，我们初学者直接就是回车，或者输入1选择默认安装即可，当然我们也可以选择自定义安装不同的版本，一些工具等等。不过这个可以以后等进阶后，自己调整修改。

## 检查是否安装成功

```rust
rustc --version
cargo --version
```

![image-20230327192954456](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230327192954456.png)



# Rust 编辑器

编辑器是可以根据个人使用习惯来选择的，无论是 Vscode，Vim，Jetbrains 的相关编辑器都可以使用Rust相关插件来进行代码编写。

我个人使用的是 `Vscode + rust-analyzer`