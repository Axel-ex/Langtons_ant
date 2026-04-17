use std::collections::HashMap;

use crate::ant::Ant;

#[derive(Default)]
pub struct Rule {
    states: Vec<usize>,
    turns: Vec<Turn>,
}

impl Rule {
    pub fn new(turn_sequence: String) -> Self {
        // TODO: parse the turn string into sequence of turn
        let turns = vec![Turn::Right, Turn::Left];
        let states = (0..turns.len()).collect();

        Self { states, turns }
    }

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
