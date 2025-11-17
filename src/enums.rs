// Enums are a type that can be one of several variants.
// They are useful for representing a value that can be one of a fixed set of options.

pub fn run() {
    enum Direction {
        Left,
        Right,
    }

    let player_direction = Direction::Left;
    let _enemy_direction = Direction::Right;

    match player_direction {
        Direction::Left => println!("Player is moving left"),
        Direction::Right => println!("Player is moving right"),
    }
}
