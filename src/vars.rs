// Variables hold primitive data or references to data
// Variables are immutable by default
// Use the `mut` keyword to make a variable mutable
// Rust is a statically typed language, which means that it must know the types of all variables at compile time
// Rust is a block-scoped language, which means that variables are only valid within the block they are defined in

pub fn run() {
    let name = "Yunis";
    let mut age = 19;

    age = 20;

    println!("Hello, I am {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Yunis", 19);
}
