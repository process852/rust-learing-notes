fn main() {
    let mut s = String::from("Hello");

    // & 表示引用，但是不会改变所有权，即类似于指针
    let length = calculate_length(&s);

    println!("{0} length is {1}", s, length);

    // 传递的是可变引用，首先变量 s 必须是可变的
    change(&mut s);
    println!("{s}");
    let r1 = &mut s;
    r1.push_str(", r1 world");

    println!{"{r1}"};

    let r2 = &mut s;
    println!("{r2}");
    
}

fn calculate_length(s: &String)->usize{
    s.len()
}

fn change(s: &mut String){
    // &mut 表示变量是可变引用
    s.push_str(", World");
}