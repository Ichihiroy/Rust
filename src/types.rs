/*
    This module is responsible for demonstrating the use of different data types in Rust.
    Integers: i8, i16, i32, i64, u8, u16, u32, u64
    Floats: f32, f64
    Booleans: bool
    Characters: char
    Tuples
    Arrays


*/

pub fn run() {
    let x = 42; // i32 by default

    let y = 3.14; // f64 by default

    let y: f64 = 1.2345678901234567890; // f64 explicitly

    // Find max size
    println!("Max size of i8: {}", std::i8::MAX);
    println!("Max size of i128: {}", std::i128::MAX);
}
