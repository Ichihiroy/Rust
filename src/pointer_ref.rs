// Pointers are used to reference memory locations directly.

pub fn run() {
    let a = 10;
    let b = &a; // b is a reference to a

    println!("Value of a: {}", a);
    println!("Reference b points to value: {}", b);

    // Dereferencing the pointer to get the value
    let c = *b;
    println!("Dereferenced value from b: {}", c);

    // For example with vectors
    let mut vec = vec![1, 2, 3];

    for x in &vec {
        println!("{}", x); // &vec gives immutable reference, no need to modify
    }

    for x in &mut vec {
        *x += 1; // dereference to modify value
    }
    println!("{:?}", vec); // [2, 3, 4]
}
