use crate::ant::Ant;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Rule {
    turns: Vec<Turn>,
}

impl Rule {
    // generic function to apply any rule
    pub fn apply(&self, ant: &mut Ant, modified_cells: &mut HashMap<(i32, i32), usize>) {
        let ant_position = ant.position();
        // find the state associated with current position.
        let cell_state = match modified_cells.contains_key(&ant_position) {
            true => {
                let cell_state = modified_cells[&ant_position];
                let next_state = (cell_state + 1) % self.turns.len();

                if next_state == 0 {
                    modified_cells.remove(&ant_position);
                } else {
                    modified_cells.insert(ant_position, next_state);
                }

                cell_state
            }
            false => {
                modified_cells.insert(ant_position, 1);
                0
            }
        };

        // the rule is in charge of applying the right rotation.
        self.turns[cell_state].apply(ant);
        ant.move_forward();
    }
}

#[derive(Debug, Error)]
pub enum ParseRuleError {
    #[error("invalid rule string (only 'L' and 'R' are allowed)")]
    InvalidChar,
}

impl std::str::FromStr for Rule {
    type Err = ParseRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut turns = Vec::new();
        for letter in s.to_lowercase().chars() {
            match letter {
                'l' => turns.push(Turn::Left),
                'r' => turns.push(Turn::Right),
                _ => return Err(ParseRuleError::InvalidChar),
            }
        }

        Ok(Self { turns })
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            turns: vec![Turn::Right, Turn::Left],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Turn {
    Right,
    Left,
}

impl Turn {
    pub fn apply(&self, ant: &mut Ant) {
        match self {
            Turn::Left => ant.turn_left(),
            Turn::Right => ant.turn_right(),
        }
    }
}
