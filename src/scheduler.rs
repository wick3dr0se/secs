use parking_lot::RwLock;
#[cfg(feature = "multithreaded")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::world::World;

pub type System = fn(&World);

#[derive(Default)]
pub struct Scheduler {
    #[cfg(feature = "multithreaded")]
    parallel_systems: RwLock<Vec<System>>,
    systems: RwLock<Vec<System>>,
}

impl Scheduler {
    #[cfg(feature = "multithreaded")]
    pub fn register_parallel(&self, system: System) {
        self.parallel_systems.write().push(system)
    }

    pub fn register(&self, system: System) {
        self.systems.write().push(system)
    }

    pub fn deregister(&self, system: System) {
        #[expect(unpredictable_function_pointer_comparisons)]
        let position = self.systems.read().iter().position(|&s| s == system);
        if let Some(pos) = position {
            let _ = self.systems.write().remove(pos);
            #[cfg(feature = "multithreaded")]
            return;
        }
        #[cfg(feature = "multithreaded")]
        #[expect(unpredictable_function_pointer_comparisons)]
        let position = self
            .parallel_systems
            .read()
            .iter()
            .position(|&s| s == system);
        #[cfg(feature = "multithreaded")]
        if let Some(pos) = position {
            let _ = self.parallel_systems.write().remove(pos);
        }
    }

    pub fn clear(&self) {
        self.systems.write().clear();
    }

    pub fn run(&self, world: &World) {
        #[cfg(feature = "multithreaded")]
        let len = self.parallel_systems.read().len();
        #[cfg(feature = "multithreaded")]
        let systems = &self.parallel_systems;
        #[cfg(feature = "multithreaded")]
        (0..len)
            .into_par_iter()
            .filter_map(|i| systems.read().get(i).copied())
            .for_each(|sys| sys(world));

        let len = self.systems.read().len();
        (0..len)
            .filter_map(|i| self.systems.read().get(i).copied())
            .for_each(|sys| sys(world));
    }
}
