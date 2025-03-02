use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::world::World;

pub type System = fn(&World);

#[derive(PartialEq)]
pub enum ExecutionMode {
    Parallel,
    Serial,
}

#[derive(Default)]
pub struct Scheduler {
    systems: Vec<(System, ExecutionMode)>,
}

impl Scheduler {
    pub(crate) fn register(&mut self, system: System, mode: ExecutionMode) {
        self.systems.push((system, mode));
    }

    pub(crate) fn deregister(&mut self, system: System) {
        if let Some(pos) = self
            .systems
            .iter()
            .position(|(s, _)| *s as *const _ == system as *const _)
        {
            self.systems.remove(pos);
        }
    }

    pub(crate) fn run(&self, world: &World) {
        self.systems
            .par_iter()
            .filter(|(_, mode)| *mode == ExecutionMode::Parallel)
            .for_each(|(sys, _)| sys(world));

        self.systems
            .iter()
            .filter(|(_, mode)| *mode == ExecutionMode::Serial)
            .for_each(|(sys, _)| sys(world));
    }
}
