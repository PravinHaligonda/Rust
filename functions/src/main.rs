fn main() {
    println!("Hello, world!");
    another_function(5, 'm');
    expression();
    plus_one(2);
}

fn another_function(x: i32, minute: char) {
    println!("The value of x: {x}{minute}");
}

fn expression() {
    // This both are statments
    let x = 3; 
    let y = 4;
    // This is an expression and return the resultant value to 7
    let z = x + y; 
    println!("the value of z: {z}");
    let a = return_values();
    println!("The value of a: {a}");
}

// Function with return values
fn return_values() -> i32 {
    5
}

// Changing expression to statment
fn plus_one(x: i32) -> i32 {
    x + 1
}