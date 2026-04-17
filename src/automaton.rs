use std::collections::HashMap;

use crate::ant::Ant;
use crate::rule::Rule;

#[derive(Default)]
pub struct Automaton {
    ant: Ant,
    modified_cells: HashMap<(i32, i32), usize>,
    rule: Rule,
    iteration: u64,
}

impl Automaton {
    pub fn new() -> Self {
        Automaton {
            ant: Ant::new(),
            modified_cells: HashMap::new(),
            rule: Rule::new("RL".to_string()),
            iteration: 0,
        }
    }

    pub fn update(&mut self) {
        self.rule.apply(&mut self.ant, &mut self.modified_cells);
        self.iteration += 1;
    }

    // skip n iteration
    pub fn skip_n_iter(&mut self, n: u64) {
        for _ in 0..n {
            self.update();
        }
    }

    // for rendering
    pub fn get_ant_position(&self) -> (i32, i32) {
        self.ant.position()
    }

    pub fn iteration(&self) -> u64 {
        self.iteration
    }

    pub fn cell_state(&self, wx: i32, wy: i32) -> usize {
        match self.modified_cells.contains_key(&(wx, wy)) {
            true => self.modified_cells[&(wx, wy)],
            false => 0,
        }
    }
}
