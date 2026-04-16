use std::collections::HashSet;

use crate::automaton::ant::Ant;

pub struct Automaton {
    ant: Ant,
    black_cells: HashSet<(i32, i32)>, // store only black cell:#![warn()]s
    iteration: u64,
}

impl Automaton {
    pub fn new() -> Self {
        Automaton {
            ant: Ant::new(),
            black_cells: HashSet::new(),
            iteration: 0,
        }
    }

    pub fn update(&mut self) {
        let ant_position = self.ant.position();

        match self.black_cells.contains(&ant_position) {
            true => {
                self.black_cells.remove(&ant_position);
                self.ant.turn_left();
            }
            false => {
                self.black_cells.insert(ant_position);
                self.ant.turn_right();
            }
        }
        self.ant.move_forward();
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

    pub fn is_black(&self, wx: i32, wy: i32) -> bool {
        self.black_cells.contains(&(wx, wy))
    }
}

#[derive(Clone)]
pub enum GridState {
    White,
    Black,
}
