extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 100. Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less    => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal   => {
                println!("You got it!");
                break;
                }
        }
    }
}
