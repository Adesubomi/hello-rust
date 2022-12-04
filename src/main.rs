enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let playerDirection:Direction = Direction::Up;

    match playerDirection {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are heading all the way down!"),
        Direction::Left => println!("We are heading left!"),
        Direction::Right => println!("We are heading towards the right!"),

    }
}