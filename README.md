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

## 数据类型(Data Types)

Rust 是一静态类型语言，必须在编译期知道所有变量的类型。

#### Scalar types

Rust有4种基本基本数据类型

* Integers

| 位长度 | 有符号数 | 无符号数
| --- | --- | ---
| 8-bit | i8 | u8
| 16-bit | i16 | u16
| 32-bit | i32 | u32
| 64-bit | i64 | u64
| 128-bit | i128 | u128
| arch | isize | usize


* Floating-point

浮点数类型存在 `f32`和`f64`两种类型，浮点数都是有符号的。


* Booleans

布尔值主要是通过`true`或`false`来表示，布尔变量默认大小是一个字节(byte)。

* Characters

Rust 中字符类型用`char`表示，使用**单引号**。Rust中`char`的大小是4字节大小，可以表示比ASCII
更多的字符。


#### 复合类型(Compound Types)

复合类型可以将多个值打包到一个类型中，Rust中有两个基本复合类型。

* Tuple

元组类型一旦创建完毕，大小不可以更改。元组可以包含**不同的数据类型**。

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

* Array

数组中所有元素类型必须一致，

```rust
// i32 表示元素类型 5表示数组个数
let a: [i32; 5] = [1,2,3,4,5];

// 数组b长度为5,元素被初始化为3
let b: [3; 5];

let first = a[0]; // 数组索引

```
