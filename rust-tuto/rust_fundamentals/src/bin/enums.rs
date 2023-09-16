
enum Direction {
    Up,
    Down,
    Left,
    Right // comma in last variant is optional
}

fn dir(d: Direction) -> i8 {
    return match d {
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
        Direction::Up => 4,
    }
}

fn main() {
    let a = Direction::Down;

    println!("{:?}", dir(a));
    println!("{:?}", dir(Direction::Up));
}