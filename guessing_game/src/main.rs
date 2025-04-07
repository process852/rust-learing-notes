use std::io; // 导入标准库模块
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Larger!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
