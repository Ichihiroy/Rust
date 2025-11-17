// Functions are declared using the `fn` keyword

pub fn run() {
    greet("Alice");
    let sum = add(5, 10);
    println!("Sum: {}", sum);

    // Closure example
    let multiply = |a: i32, b: i32| a * b;
    let product = multiply(4, 6);
    println!("Product: {}", product);

    let result = add(5, 10);
    println!("The result of adding 5 and 10 is: {}", result);
    greet("Bob");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return
}
