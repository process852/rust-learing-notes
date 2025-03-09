fn main() {
    let x = 5;
    let mut y = 5;
    println!("The value of x is: {x}");
    // x = 6; error
    println!("The value of x is: {x}");

    println!("The value of y is: {y}");
    y = 0;
    println!("The value of y is: {y}");
    
    // 覆盖机制
    let z = 5;
    let z = z + 1;
    println!("The value of z is: {z}");

    {
        let z = z * 2;
        println!("The value of z is: {z}");
    }
    println!("The value of z is: {z}");
}
