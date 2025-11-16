pub fn run() {
    let person: (&str, &str, i8) = ("Alice", "Smith", 30);

    println!("First Name: {}", person.0);
    println!("Last Name: {}", person.1);
    println!("Age: {}", person.2);
}
