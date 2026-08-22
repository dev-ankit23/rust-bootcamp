use rand::Rng;
use std::io;
fn main() {
    println!("Guess the Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please enter your guess");
    println!("The secret number is :{secret_number}");
    println!("Input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
