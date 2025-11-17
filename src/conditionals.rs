pub fn run() {
    let age = 18;

    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    let is_student = true;

    if is_student {
        println!("You get a student discount.");
    } else {
        println!("No discount available.");
    }

    let score = 85;

    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };

    println!("Your grade is: {}", grade);

    // Ternary-like conditional assignment
    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
}
