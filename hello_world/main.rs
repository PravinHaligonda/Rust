use std::io;

fn main() {
    println!("Hello, World");
    let mut num = Integer::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error");

    println!("Num: {}", num);
}
