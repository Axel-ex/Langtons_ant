use crate::ant::{Ant, Turn};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
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
        ant.turn(self.turns[state]);

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

#[derive(Debug, PartialEq, Error)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ant::Direction;

    use super::*;

    #[test]
    fn empty_rule() {
        let rule = Rule::from_str("");

        assert_eq!(rule, Err(ParseRuleError::EmptyRule));
    }

    #[test]
    fn invalid_char() {
        let rule = Rule::from_str("LLRK");

        assert_eq!(rule, Err(ParseRuleError::InvalidChar));
    }

    #[test]
    fn apply_rule() {
        let rule = Rule::from_str("RL").unwrap();

        let mut ant = Ant::new(Direction::Up); // create rule from direction and position
        let mut modified_cells = HashMap::new();

        rule.apply(&mut ant, &mut modified_cells);
        //check modified modified_cells
        assert_eq!(modified_cells.len(), 1);
        assert_eq!(ant.direction(), Direction::Right);
        assert_eq!(ant.position(), (1, 0));
    }
}
