use crate::ant::Ant;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Rule {
    name: String,
    turns: Vec<Turn>,
}

impl Rule {
    pub fn apply(&self, ant: &mut Ant, modified_cells: &mut HashMap<(i32, i32), usize>) {
        let pos = ant.position();

        // read current state (if not in the map, default state = 0)
        let state = modified_cells.get(&pos).copied().unwrap_or(0);

        // apply turn based on current state
        self.turns[state].apply(ant);

        // compute next state (nb of state possible = turns.len())
        let next_state = (state + 1) % self.turns.len();

        if next_state == 0 {
            modified_cells.remove(&pos);
        } else {
            modified_cells.insert(pos, next_state);
        }

        ant.move_forward();
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Error)]
pub enum ParseRuleError {
    #[error("invalid rule string (only 'L' and 'R' are allowed)")]
    InvalidChar,

    #[error("empty rule string")]
    EmptyRule,
}

impl std::str::FromStr for Rule {
    type Err = ParseRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRuleError::EmptyRule);
        }

        let mut turns = Vec::new();
        for letter in s.to_lowercase().chars() {
            match letter {
                'l' => turns.push(Turn::Left),
                'r' => turns.push(Turn::Right),
                _ => return Err(ParseRuleError::InvalidChar),
            }
        }

        Ok(Self {
            name: s.to_string(),
            turns,
        })
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            name: "RL".to_string(),
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
