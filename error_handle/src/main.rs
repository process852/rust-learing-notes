fn main() {
    option_example1();

    option_example2();
}
fn option_example2() {
    let s1: Option<String> = Some(String::from("Hello world!"));
    let s2: Option<String> = None;
    let s3 = s1.clone();

    // s1 为 Some时，expect 返回其中的内容
    assert_eq!(s1.expect("s1 is None"), "Hello world!");
    // s2 为 None 时，expect 返回错误信息, 并且输出错误信息
    // assert_eq!(s2.expect("s2 is None"), "s2 is None");

    // s1 为 Some 时，unwrap 返回其中的内容
    // s1 为 None时，出现 Panics 终止程序执行
    // s1 此处不可以再用，之前 expect 已经拿走字符串的所有权
    assert_eq!(s3.unwrap(), "Hello world!");

    // s2 为 None时，返回提供的默认值
    assert_eq!(s2.unwrap_or("hello".to_string()), "hello");

    let s4: Option<String> = Some(String::from("HaHaHa"));
    match s4 {
        Some(s) => println!("s2 is {}", s),
        None => println!("s2 is None")
    }

}

fn option_example1() {
    let s: Option<String> = None;
    let x: Option<i32>  = Some(5);
    // 如果 s 为None, 则返回True
    if s.is_none(){
        println!("s is None");
    } else {
        println!("s is Some");
    }
    // 如果 x 为 Some, 则返回true
    if x.is_some() {
        println!("x is Some");
    } else {
        println!("x is None");
    }
}
