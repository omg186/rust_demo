enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn which_way(direction: Direction) {
    match direction {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    };
}
fn main() {
    println!("Hello, world!");
    which_way(Direction::Down);
    which_way(Direction::Up);
    which_way(Direction::Left);
    which_way(Direction::Right);
}
