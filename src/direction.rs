pub enum Direction {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn from_u8(value: u8) -> Direction {
        match value {
            0 => Direction::NONE,
            1 => Direction::UP,
            2 => Direction::DOWN,
            3 => Direction::LEFT,
            4 => Direction::RIGHT,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
