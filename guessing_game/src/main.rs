use std::io;

fn main() {
    // Ask for user input
    println!("Guess the number");
    println!("Please input your guess: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");

    // Greet them and exit.
    println!("You guessed: {}", guess);
}

// Check the input is in expected form
// Expected form: Number between 1 - 100
// Check whether the guess is too low or too high
// If the guess guess in correct. 

// Explaination of what above code does. Let's move on.
fn check() {
    // all variables starts with let
    // mut is wheter the variable is mutable of immutable.
    // guess is name of the variable
    // String::new() creates empty new instance of type string.
    let mut guess = String::new()

    io::stdin()
        // .method_name()
        // Return: Result -> (Ok and Err)
        // Ok: Operation successful
        // Err: Operation failed -> Why the operation failed
        .read_line(&mut guess) 
        // Will cause the program to crash and display the message.
        .expect("Faild to read the line");
}