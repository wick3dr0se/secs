use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::world::World;

pub type System = fn(&World);

#[derive(PartialEq)]
pub enum ExecutionMode {
    Manual,
    Parallel,
    Serial
}

#[derive(Default)]
pub struct Scheduler {
    systems: Vec<(System, ExecutionMode)>
}

impl Scheduler {
    pub fn register(&mut self, system: System, mode: ExecutionMode) {
        self.systems.push((system, mode));
    }

    pub fn run(&self, world: &World) {
        self.systems.par_iter()
        .filter(|(_, mode)| *mode == ExecutionMode::Parallel)
        .for_each(|(sys, _)| sys(world));

        self.systems.iter()
            .filter(|(_, mode)| *mode == ExecutionMode::Serial)
            .for_each(|(sys, _)| sys(world));
    }
}