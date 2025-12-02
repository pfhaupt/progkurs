enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right"
        }
    }
}

pub fn main() {
    println!("up: {}", Direction::Up.as_str());
    println!("down: {}", Direction::Down.as_str());
    println!("left: {}", Direction::Left.as_str());
    println!("right: {}", Direction::Right.as_str());
}
