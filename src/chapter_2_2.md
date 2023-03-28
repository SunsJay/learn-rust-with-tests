# Hello, World

查看本章的所有代码

按照传统，我们用新语言编写的第一个程序都是 Hello,World。
代码存放的位置根据自己的喜好，并没有固定的规定的存放一说，在这里我们不介绍使用 `rustc` 进行编译运行，而是使用 `Cargo` 来管理和运行我们的程序。

## 创建程序

打开终端，跳转到存放自己代码的目录中，然后输入 `cargo new [project_name]`，如下图创建一个`hello`的应用程序项目，如果你想创建一个库，命令带上 `--lib` ，这个后面用到的时候再关心，现在跳出。

![image-20230327194152325](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230327194152325.png)



## 程序结构

`cargo` 会默认给你创建一个 `打印Hello,World` 的应用程序，如果你配置文件没有关闭 `git` 初始化功能，它还会在创建项目的时候，自动将该项目 `git` 初始化

![image-20230327194351957](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230327194351957.png)

```rust
fn main() {
    println!("Hello, world!");
}
```



## 它是如何运行

用 Rust 编写程序，我们可以使用rust编译rs文件，并执行，只要它含有main方法，这个和C语言是高度相似的。不过  Rust 中有 `Cargo`。 它是Rust的构建系统和包管理器，用来管理我们的项目，和从远程仓库里获取添加一些依赖的库到我们的项目中，以及对依赖进行版本管理。

除了之前说的 `cargo new`  命令，这可以为我们创建一个项目，可以是二进制可执行文件，也可以是库。还有常见的 `cargo build cargo run cargo test ` 等等相关命令，帮助我们构建，运行，测试项目。



## 如何测试

如果创建的是一个库的话，会发现它不是帮我们自动生成了打印 `Hello, World!` 程序。而是生成了一个`两数相加`的方法， 并对它进行单元测试，验证是否等于两数相加之和

### 单元测试

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

```

我们可以看到它是如何编写测试的，首先需要定义一个`模块(mod)`，名称并不一定是需要tests，你也可以定义为其他名字，但是最重要的是需要 `#[cfg(test)]`这个一个注释放在模块前面，这个注释告诉编译器，只有在运行 `cargo test` 的时候才编译和运行，在 `cargo build ` 或者 `cargo run` 的时候会忽略。

+ `cfg` 代表配置，告诉Rust只在给定的特定配置选项下才包含以下项目

带有 `#[test]` 注释的函数，会方便我们查看如果是多个测试的时候，具体是哪个测试函数没有通过。

### 集成测试

集成测试是在项目`根目录`下创建 `tests` 文件夹，Cargo会自动去这个文件夹中寻找集成测试文件。不需要我们写 `#[cfg(test)]` 注释。



### assert

+ `assert!()`		检查里面的是否为真
+ `assert_eq!()`    检查两者是否相等
+ `assert_ne!()`    检查两者是否不相等



## Hello,You

现在有了测试，就可以安全地迭代我们的软件了。

现在我们将**先开始编写测试**。

我们的下一个需求是指定问候的对象。

让我们从在测试中捕获这些需求开始。这是基本的测试驱动开发，可以确保我们的测试用例 *真正*在测试我们想要的功能。当你回顾编写的测试时，存在一个风险：即使代码没有按照预期工作，测试也可能继续通过。

创建一个用于单元测试的库，我们这里创建的 `hello_test`

将红框里的代码**删除**

![image-20230328190630754](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230328190630754.png)

`code/chapter2/hello_test/src/lib.rs`

删完后只剩下一个测试模块

```rust
#[cfg(test)]
mod tests {
    use super::*;

}
```

我们先写测试部分代码

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let left = hello("Chris".to_string());
        let right = String::from("Hello, Chris");
        assert_eq!(left, right);
    }
}

```

运行 `cargo test` ，这时候肯定会报错，因为我们并没有 `hello` 这个函数。 

![image-20230328194155645](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230328194155645.png)

创建`hello` 函数，接受一个String类型的参数，并返回String类型

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let left = hello("Chris".to_string());
        let right = String::from("Hello, Chris");
        assert_eq!(left, right);
    }
}

fn hello(s: String) -> String {
    format!("Hello, {}", s)
}

```

运行测试，这时候可以看出测试能够通过了！

![image-20230328194322075](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/image-20230328194322075.png)