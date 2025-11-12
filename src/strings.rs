pub fn run() {
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    // Add a character
    hello.push('W');
    println!("{}", hello);

    // Add a string
    hello.push_str("orld!");
    println!("{}", hello);

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Replace substring
    let new_hello = hello.replace("World", "There");
    println!("{}", new_hello);

    // Check for substring
    println!("Contains 'World'? {}", hello.contains("World"));

    // Create string with capacity
    let s = String::with_capacity(10);

    assert_eq!(s.capacity(), 10);
}
