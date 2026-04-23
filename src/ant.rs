pub struct Ant {
    position: (i32, i32),
    direction: Direction,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    Right,
    Left,
}

impl Ant {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
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

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn turn_ant_right() {
        let mut ant = Ant::default(); // direction = right

        let turn = Turn::Right;
        ant.turn(turn);
        assert_eq!(ant.direction(), Direction::Down);
        ant.turn(turn);
        ant.turn(turn);
        assert_eq!(ant.direction(), Direction::Up);
    }

    #[test]
    pub fn turn_ant_left() {
        let mut ant = Ant::default(); // direction = right

        let turn = Turn::Left;
        ant.turn(turn);
        assert_eq!(ant.direction(), Direction::Up);
        ant.turn(turn);
        ant.turn(turn);
        assert_eq!(ant.direction(), Direction::Down);
    }

    #[test]
    fn move_forward() {
        let mut ant = Ant::default();

        let prev_position = ant.position();
        ant.move_forward();
        let curr_position = ant.position();
        assert_eq!(prev_position.0 + 1, curr_position.0)
    }
}
