fn main() {
    println!("Hello, world!");

    // addition 
    let x = 5 + 10;

    // subtraction 
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("trauncated {truncated}");

    // remainder
    let remainder = 43 % 5;

    let c: char = 'a';
    println!("char is {c}");
    let t = false;
    println!("bool false is {t}");

    let tup: (char, u8, f32) = ('z', 1, 3.2);

    println!("tuple: ({0}, {1}, {2})", tup.0, tup.1, tup.2);
    let (c1, u1, f1) = tup;
    println!("tuple: ({c1}, {u1}, {f1})");

    let array: [f32; 5] = [2.3, 2.0, 3.1, 4.0, 5.2];
    println!("array[2] = {0}", array[2]);
}
