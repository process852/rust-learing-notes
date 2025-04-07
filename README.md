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

## 函数(function)

函数使用关键字`fn`来标识。Rust 不关心函数定义的先后顺序，只要函数在调用函数可见的作用域即可。


#### 参数(parameters)

* Rust 中函数参数必须声明参数的类型

#### Statements and Expressions

* 语句(statements)是执行某些操作不返回值的指令
* 表达式(expressions)是存在评估结果值的,**表达式不包含结束的分号**。

```rust
let y = 3; // 此处是语句，并不返回值

// 函数调用是一个表达式
// 宏调用也是一个表达式
// 花括号包裹的作用域快也是一个表达式

let x = {
	let x = 3;
	x + 1
}
```

#### 函数返回值

函数返回类型声明使用`->`，函数返回值等于函数体花括号内最后一个表达式的值。

```Rust
// 返回值为5, 类型为 i32
fn five() -> i32 {
	5
}
```

## 注释

Rust 使用`//`进行行注释

## 控制流(Control Flow)

#### if 表达式
```Rust
fn main { 
	let number = 3;
	
	if number < 5 {
		println!("condition was true");
	} else {
		println!("condition was false");
	}
}
```

#### Loops

Rust 存在三种形式的循环。

* loop

```Rust
fn main() {
	let mut count = 0;
	// loop 可以使用标签说明指代那个loop，标签名需要使用单引号开头
	'counting_up: loop {
		println!()
	}
}
```

* while

* for

```Rust
fn main() {
	let a = [10, 20, 30, 40];

	for element in a {
		println!("The value is: {element}");
	}
}
```

## 所有权(Ownership)

所有权特性是Rust中的核心特征，用于保证在没有垃圾回收的前提下保证内存安全。所有权是一套
规则集合用于管理内存。

Owership Rules:

* 每个值拥有一个拥有者
* 一次只能拥有一个拥有者
* 当拥有者超出作用域范围，值会被丢弃

Variable Scope:

当变量进入作用域开始有效，直到走出作用域范围。

```Rust
{
	let s = "hello"; // s 开始有效
}  // 走出作用域后 s 不再有效
```

#### Memory and Allocation

```Rust
let s1 = String::from("hello");

// 本质上是拷贝了s1中指向字符串的指针
// Rust 会自动使得 s1 无效，这种称为 move 行为
// 大大减少了内存数据的拷贝
let s2 = s1;


let mut s = String::from("hello");
// 对于已经初始化的变量值，重新赋值时
// Rust 通过drop函数立即释放"hello"的内存
s = String::from("ahoy");

```

## References and Borrowing

```Rust
fn main() {
    let s = String::from("Hello"); // 变量 s 仍保持对于字符串的所有权

    // & 表示引用，但是不会改变所有权，即类似于指针
    let length = calculate_length(&s);

    println!("{0} length is {1}", s, length);
}

fn calculate_length(s: &String)->usize{
	// 由于 s 是对于字符串的借用，无法修改字符串内容
    s.len()
}
```

* mutable reference(可变引用)
	* 同一时刻不可以可变引用多次(防止对于数据的同时修改，避免数据竞争)
	* 但是不再同一时刻发生的可变引用是可行的
	* 不可以同时拥有可变引用和不可变引用
	* 多个不可变引用是可行的，即不可变引用只具有读数据的权限
	* 引用的作用域范围从被引入到最后一次该引用被使用时结束

```Rust
fn main() {
    let mut s = String::from("Hello");
    let r1 = &mut s;

    println!{"{r1}"};

    let r2 = &mut s;
    println!("{r2}");
    
}

```

* Dangling References(悬空引用)
引用总是指向有效的内存地址。


## Packages and Crates

* 一个crate 是编译器考虑的最小代码组织单元
* crate 可以包含 modules 
* 一个crate 可以以两种形式存在，二进制crate或库crate
	* 包含 main 的crate是可执行文件
	* 库crate不包含main函数
* package 是包含一个或多个crates
	* 一个package 包含一个 `Cargo.toml` 文件用于描述如何构建包中的`crates`
	* 一个package 可以包含许多二进制`crates`,但是至多只有一个库`crate`
* `src/main.rs`是二进制crate的root，`src/lib.rs`是库crate的root.
* 一个package 可以放置多个二进制`crate`在`src/bin`目录下

#### 模块和路径系统

编译器是如何工作的关于模块和路径：

* 首先查找 `src/main.rs` 和 `src/lib.rs` 两个根 crate
* 在 crate root file中我们可以声明新的模块，·例如`mod garden;`
* 然后编译器在以下路径搜索模块的代码
	* 内嵌模式 `mod garden {...}`
	* 文件 `src/garden.rs`
	* 文件 `src/garden/mod.rs`
* 可以在非root crate 文件中声明子模块`mod vegetables;`
	* 內联模式
	* `src/garden/vegetables.rs`
	* `src/garden/vegetables/mod.rs`
* 同一个module内的所有内容可以被其他部分使用，只要遵循权限规则
	* `crate::garden::vegetables::某个对象`
* 私有和公有属性
	* 默认情况下子模块内容对于父模块是不可见的
	* 使用关键字`pub`改变权限
* `use` 关键字可以减少过长路径的编写


路径(PATH):

为了指示如何在模块树中找到所需的对象，需要提供对象的路径(类似于文件系统)。

* 绝对路径：对于外部crate的代码，绝对路径开始于crate name。对于当前crate,则使用`crate`
* 相对路径：相对当前模块，使用`self`/`super`等关键字或者目前模块的标识符


