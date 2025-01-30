use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::World;

pub type System = fn(&World);

#[derive(Default)]
pub struct Scheduler {
    systems: Vec<System>
}

impl Scheduler {
    pub fn register(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn run(&self, world: &World) {
        for system in &self.systems {
            system(world);
        }
    }

    pub fn run_par(&self, world: &World) {
        self.systems.par_iter().for_each(|system| system(world));
    }
}