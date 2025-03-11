fn main() {
    another_function(5);
    let x = five();
    println!("five return value is: {x}");

    if x > 6 {
        println!("x > 6");
    }else{
        println!("x <= 6");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter*2;
        }
    };

    println!("Result = {result}");

    let a = [10, 20, 30, 40];

    for element in a {
         println!("The value is: {element}");
    }

    let mut index = 0;

    while index < 10 {
        println!("The index is {index}");
        index += 1;
    }
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
