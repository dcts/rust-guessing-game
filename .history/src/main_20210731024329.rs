use std::io;

fn main() {
    println!("=== Guess the number game ===");
    println!("please insert your guess: ");

    let mut guess = String::new();
    const Y: char = '2';

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("you guessed {}. \ny is {}", guess, Y);
}
