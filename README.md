# rust-learing-notes
Learning Rust !!!

## 安装教程

## Cargo 基本用法

Cargo 是一个构建系统和包管理工具。

```bash
// 创建一个新的包，其在目录hello_cargo下
cargo new hello_cargo

// 构建当前项目，生成内容在 target 目录下
// 默认生成 debug 模式的可执行文件
cargo build

// 将构建和执行可执行文件合并为一步
cargo run
``` 

## Rust 变量和可变性

Rust 中变量默认是不可变的，如何理解？

```rust
let x = 5; // 默认情况下 x 变量是不可变的

// x = 6; 在编译时会报错

let mut y = 1; // 关键字 mut 表示该变量是可修改的
```

## 常量(constant)

常量是指值被绑定到一个名字上，也不被允许修改。但是常量和不可变变量存在细微差别：

* 常量总是不可变，即不可以使用`mut`关键字
* 常量可以在任何作用域被声明
* 常量的值可以是常量表达式(constant expression)

```rust
// 常量名字为 THREE_HOURS_IN_SCONDS
const THREE_HOURS_IN_SCONDS: u32 = 60 * 60 * 3;
```

## Shadowing 机制

理解为同名变量的覆盖，可以是不同类型的同名变量的覆盖，必须用`let`关键字声明。
当作用域结束时，恢复之前的同名变量的可见性。
