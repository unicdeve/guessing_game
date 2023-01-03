// use std::io;
// use std::cmp::Ordering;
use std::{cmp::Ordering, io}; // using common parts
use rand::Rng;

fn main() {
    println!("Guessing the number game!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
