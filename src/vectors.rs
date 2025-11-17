// Vectors are growable arrays in Rust. They can store multiple values of the same type.

pub fn run() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Print the vector
    println!("Vector: {:?}", vec);

    // Accessing elements
    println!("First element: {}", vec[0]);

    // Length of the vector
    println!("Length of the vector: {}", vec.len());

    // Iterating over the vector
    for element in &vec {
        println!("Element: {}", element);
    }

    // Iterating and mutating over the vector
    let mut vec2 = vec![1, 2, 3, 4, 5];
    for el in vec2.iter_mut() {
        *el *= 2;
    }

    // Slicing the vector
    println!("Slice: {:?}", &vec[1..4]);

    // Mutable vector
    let mut mut_vec: Vec<i32> = vec![10, 20, 30];
    mut_vec[0] = 100;
    println!("Mutable Vector: {:?}", mut_vec);
}
