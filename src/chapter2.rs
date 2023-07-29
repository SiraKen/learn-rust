//! Chapter 2
//!
//! 数当てゲームのプログラミング

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 * 数当てゲーム
 */
pub fn __main__() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..11);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
