fn main() {
    let guess: i32 = "42".parse().expect("Not a number");
    println!("Guess: {guess}");
    signed_int();
    unsigned_integer();
    number_literals();
    floating_point();
}

fn signed_int() {
    // Signed 8 bit integer
    let _a: i8 = 10;

    // Signed 16 bit integer
    let _b: i16 = 20;

    // Signed 32 bit integer
    let _c: i32 = 30;

    // Signed 64 bit integer
    let _d: i64 = 40;

    // Signed 128 bit integer
    let _e: i128 = 50;

    // "arch":64 bit / 32 bit architecture
    let _f: isize = 60;
}

fn unsigned_integer() {
    // Unsigned 8 bit integer
    let _a: u8 = 3;

    // Unsigned 16 bit integer
    let _b: u16 = 4;

    // Unsigned 32 bit integer
    let _c: u32 = 5;

    // Unsigned 64 bit integer
    let _d: u64 = 6;

    // Unsigned 128 bit integer
    let _e: u128 = 7;

    // "arch": 64 bit / 32 bit architecture
    let _f: isize = 8;

}

fn floating_point() {
    // 32 bit single precision floating point number
    let _a: f32 = 1.1;

    // 64 bit double precision floating point number
    let _b: f64 = 1.2;
}

fn number_literals() {
    let one_thousand = 1_000;
    println!("One Thousand: {one_thousand}");
}