fn main() {
    let num = 3;

    if num < 5 {
        println!("Condition is true");
    } else {
        print!("Condition is false");
    }

    check(20);
}

fn check(n: i32) {
    if n < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is greater than 10");
    }
}
