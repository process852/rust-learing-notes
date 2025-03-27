fn main() {
    let user = User{
        active: true,
        username: String::from("codejin"),
        email: String::from("2301165479@qq.com"),
        sign_in_count: 1,
    };
    // 用点号获取对应的字段
    println!("user:{2}, email:{1}, active:{0}, count:{3}", user.active, user.email, 
        user.username, user.sign_in_count);

    // 利用user的剩余字段内容更新user2
    // 而且 ..user必须是最后一个字段，后面不可以加逗号
    let user1 = User{
        email: String::from("qq.com"),
        ..user
    };
    println!("user:{2}, email:{1}, active:{0}, count:{3}", user1.active, user1.email, 
        user1.username, user1.sign_in_count);
    // 无法再访问 user, 其中的字符串已经移动到 user1中
    // println!("user:{2}, email:{1}, active:{0}, count:{3}", user.active, user.email, 
    //     user.username, user.sign_in_count);
    let point = Point(1,2,3);
    println!("{}-{}-{}", point.0, point.1, point.2);
}

// 结构体定义不需要分号结尾
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Point(i32, i32, i32);