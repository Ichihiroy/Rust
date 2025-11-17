// Structs are similar to classes in other languages. They are used to create custom data types that group related data together.

pub fn run() {
    struct Person {
        name: String,
        age: u8,
    }

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} is {} years old.", person1.name, person1.age);

    // You can also implement methods for structs
    impl Person {
        // Constructor method
        fn new(name: String, age: u8) -> Person {
            Person { name, age }
        }

        fn greet(&self) {
            println!(
                "Hello, my name is {} and I am {} years old.",
                self.name, self.age
            );
        }
    }

    let person2 = Person::new(String::from("Bob"), 25);
    person2.greet(); // Hello, my name is Bob and I am 25 years old.
}
