fn main() {

    // Immutable Variable (by default)
    let y = 9;
    println!("The value of y: {y}");

    // Mutable Variable
    let mut x = 3;
    println!("The value of x: {x}");
    x = 4;
    println!("The value of x: {x}");

    // Constants
    // Always Immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Seconds in Three Hours: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let p = 1;
    println!("P: {p}");
    let p = p + 1;
    println!("P: {p}");

    {
        let p = p + 3;
        println!("The value of p in inner scope is {p}");
    }

    let mut m = 2;
    println!("M: {m}");
    let mut m = m + 2;
    println!("M2: {m}");
}

