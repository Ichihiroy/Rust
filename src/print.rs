pub fn run() {
    // Print to the console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Yunis", "Baku");

    // Positional args
    println!("{0} is from {1} and he loves {1}", "Ismail", "Baku");

    // Named args
    println!(
        "{name} likes to play {activity}",
        name = "Fuad",
        activity = "Football"
    )
}
