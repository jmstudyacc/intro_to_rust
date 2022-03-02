extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Rusty Guessing Game!");
    println!("A random target number will be generated. It is your task to guess what that target number might be!");
    println!("You will be prompted to enter your guess via the terminal");
    'game_loop: loop {
        let target = rand::thread_rng().gen_range(1..=100);
        let mut count = 0;

        while count < 5 {
            let mut guess = String::new();
            println!("Please enter your guess: ");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("ERROR: Please enter a number.\n");
                    continue;
                }
            };

            match guess.cmp(&target) {
                Ordering::Less => {
                    println!("Low");
                    count += 1;
                    continue;
                }
                Ordering::Greater => {
                    println!("High");
                    count += 1;
                    continue;
                }
                Ordering::Equal => {
                    println!("You win! Congratulations!");
                    break;
                }
            };
        }
        println!("\nGame over, try again!\n");
    }
}
