pub struct Ant {
    position: (i32, i32),
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Ant {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }
}

impl Default for Ant {
    fn default() -> Self {
        Ant {
            position: (0, 0),
            direction: Direction::Right,
        }
    }
}
