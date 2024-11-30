// Use the input/output library from standard library.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Explaination of what above code does. Let's move on.
fn main() {
    // Ask for user input
    println!("Guess the number");

    // thread_rng() function gives us the particular random number generator
    // gen_range(start..=stop) 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess: ");

    // all variables starts with let
    // mut is wheter the variable is mutable of immutable.
    // guess is name of the variable
    // String::new() creates empty new instance of type string.
    let mut guess = String::new();
    io::stdin()
        // Return: Result -> (Ok and Err)
        // Ok: Operation successful
        // Err: Operation failed -> Why the operation failed
        .read_line(&mut guess)
        // Will cause the program to crash and display the message.
        .expect("Failed to read the line.");

    // Shadowing the variable
    let guess: u32 = guess.trim().parse().expect("Please type a number.");

    // Greet them and exit.
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}

// Check the input is in expected form
// Expected form: Number between 1 - 100
// Check whether the guess is too low or too high
// If the guess guess in correct. 
