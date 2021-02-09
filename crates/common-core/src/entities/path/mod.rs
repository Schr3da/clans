use specs::prelude::*;
use specs_derive::*;

use crate::entities::traits::properties::*;

use crate::entities::traits::render::*;

pub type PathBuilder = Path;

#[derive(Clone, PartialEq, Eq, Component)]
pub struct Path {
    pub start: usize,
    pub steps: Vec<usize>,
    pub end: usize,
    pub needs_update: bool,
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

impl Updateable for Path {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl Path {
    pub fn new(steps: Vec<usize>) -> Self {
        let current = 0;
        let length = steps.len();
        let needs_update = false;

        if length == 0 {
            return Path {
                start: 0,
                steps: Vec::new(),
                end: 0,
                current,
                length,
                needs_update,
            };
        }

        Path {
            start: steps[current],
            steps: steps.clone(),
            end: steps[length - 1],
            current,
            length,
            needs_update,
        }
    }

    pub fn set(&mut self, steps: Vec<usize>) {
        self.current = 0;
        self.length = steps.len();

        if self.length == 0 {
            self.start = 0;
            self.steps = Vec::new();
            self.end = 0;
            return;
        }

        self.start = steps[self.current];
        self.steps = steps.clone();
        self.end = steps[self.length - 1];
    }
}
