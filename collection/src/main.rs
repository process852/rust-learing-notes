
fn main() {
    show_vectors();
    show_string();
}

fn show_string() {
    let data: i32 = 32; // 栈区的数据仍然存在

    // data 值从栈上拷贝至堆区
    let data_string = data.to_string();

    println!("data string: {data_string}");
    println!("data {data}");

    let hello = String::from("Hello World!");
    println!("{hello}");

    // 字符串的内容更新
    let mut s = String::from("foo");
    // s_slice 是一个字符串切片
    let s_slice = "bar";
    // push_str 参数是一个字符串切片的引用
    s.push_str(s_slice);
    println!("s_slice: {s_slice}"); // s_slice 仍然拥有所有权
    println!("{s}");

    // push 方法追加一个字符
    s.push('c');
    println!("{s}");

    // 2. 利用 + 运算符实现字符串拼接
    let s1 = String::from("Hello, "); // s1所有权转移至 s3
    let s2 = String::from("World!"); // s2保持不变
    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s2}");
    // println!("{s1}"); error

    // 3. 使用format!宏来拼接字符串 不会获取字符串的所有权
    let s4 = format!("{s3}-{s2}");
    println!("{}", s4);
    println!("{}", s3);

    // Rust 不允许对字符串进行索引获取字符
    // 不同的编码方式导致可能是一个字节/字符/字形簇等
    // 字符串切片
    // 表示获取前4个字节内容
    let s5 = &s4[0..4]; // s5 表示字符串切片，类型是 &str


    // 4. 迭代遍历字符串
    for c in s5.chars(){
        // 获取每个字符
        println!("{c}");
    }

    for b in s5.bytes(){
        // 获取每个字节数据
        println!("{b}");
    }



}


fn show_vectors() {
    // 创建一个 vector 
    // <> 内的类型表明了 vector 的元素类型
    let mut v: Vec<i32> = Vec::new();

    // 利用宏创建列表元素
    let v1 = vec![1, 2, 3, 0];

    // push 方法往容器内添加元素
    // v 必须是一个 mut 可变容器
    v.push(5);

    // 读取容器元素
    // 方法一：利用索引获取元素
    // 方法二：利用get方法获取元素

    // & 表示引用容器中的元素
    let third: &i32 = &v1[3];
    println!("Third element is {third}");

    let first: Option<&i32> = v.get(0);
    match first {
        Some(third) => println!("v first element {third}"),
        None => println!("None"),
    }
    // 迭代遍历容器元素
    // vec![1,2,3,4,5] 返回的类型是一个容器类型
    let v2: Vec<i32> = vec![1,2,3,4,5];

    for item in &v2 {
        println!{"{item}"};
    }

    // 迭代遍历可变容器元素
    let mut v3: Vec<i32> = Vec::new();
    v3.push(2);
    v3.push(1);
    v3.push(5);

    // * 表示解引用操作符
    for item in &mut v3{
        *item += 10;
    }

    for item in &v3 {
        println!("v3: {item}");
    }

    // Vector 容器只能存储相同类型的元素
}
