use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the securet number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() // io::stdin::Stdinインスタンスを返す
            .read_line(&mut guess) // io::Resultを返す
            .expect("Failed to read line"); // io::Resultのexpect()メソッド

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter number!");
                continue;
            }
        };

        println!("You guessd: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("ok!");
                break;
            }
        }
    }
}
