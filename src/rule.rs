use std::collections::HashMap;

use crate::ant::Ant;

pub struct Rule {
    states: Vec<usize>,
    turns: Vec<Turn>,
}

impl Rule {
    pub fn new(turn_sequence: String) -> Self {
        // TODO: parse the turn string into sequence of turn
        let turns = vec![Turn::Right, Turn::Left];
        let states = vec![0, 1];

        Self { states, turns }
    }

    // generic function to apply any rule
    pub fn apply(&self, ant: &mut Ant, modified_cells: HashMap<(i32, i32), usize>) {
        let ant_position = ant.position();
        // find the state associated with current position.
        let cell_state = match modified_cells.contains_key(&ant_position) {
            true => modified_cells[&ant_position],
            false => 0,
        };

        // the rule is in charge of applying the right rotation.
        self.turns[cell_state].apply(ant);
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
