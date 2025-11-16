// Arrays are fixed-size collections of elements of the same type in Rust.

pub fn run() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Print the array
    println!("Array: {:?}", arr);

    // Accessing elements
    println!("First element: {}", arr[0]);

    // Length of the array
    println!("Length of the array: {}", arr.len());

    // Iterating over the array
    for element in arr {
        println!("Element: {}", element);
    }

    // Slicing the array
    let slice = &arr[1..4];
    println!("Slice: {:?}", slice);

    // Mutable array
    let mut mut_arr: [i32; 3] = [10, 20, 30];
    mut_arr[0] = 100;
    println!("Mutable Array: {:?}", mut_arr);
}
