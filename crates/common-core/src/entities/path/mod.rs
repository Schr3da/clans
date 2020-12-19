use specs::prelude::*;
use specs_derive::*;

use crate::entities::traits::properties::*;

#[derive(Clone, PartialEq, Eq, Component)]
pub struct Path {
    pub start: usize,
    pub steps: Vec<usize>,
    pub end: usize,
    length: usize,
    current: usize,
}

impl WalkableProperty for Path {
    fn did_reach_end(&self) -> bool {
        self.length == 0 || self.current == self.length - 1
    }

    fn current_step(&self) -> Option<usize> {
        match self.did_reach_end() {
            false => Option::Some(self.steps[self.current]),
            true => Option::None,
        }
    }

    fn move_forward(&mut self) {
        if self.did_reach_end() {
            return;
        }

        self.current += 1;
    }
}

impl Path {
    pub fn new(steps: Vec<usize>) -> Self {
        let current = 0;
        let length = steps.len();

        if length == 0 {
            return Path {
                start: 0,
                steps: Vec::new(),
                end: 0,
                current,
                length,
            };
        }

        Path {
            start: steps[current],
            steps: steps.clone(),
            end: steps[length - 1],
            current,
            length,
        }
    }
}
