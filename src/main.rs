extern crate rand;

use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Welcome!");
    play()
}

fn play() {
    let rand_int = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please enter a number");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = guess.trim().parse().expect("Invalid number.");
        if guess == rand_int {
            println!("You won!");
            break;
        }
        println!("Try again!")
    }
    println!("Would you like to play again? [y/n]");
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Failed to read line");
    if ["yes", "y", "ya", "yeah"].contains(&answer.to_lowercase().trim()) {
        println!("Hello, again!");
        play()
    }
}