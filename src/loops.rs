// Loops are used to execute a block of code multiple times.

pub fn run() {
    // Infinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);
        if count >= 5 {
            break;
        }
    }

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    // For loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Element: {}", element);
    }

    // For loop with range
    for i in 1..6 {
        println!("i: {}", i);
    }
}
