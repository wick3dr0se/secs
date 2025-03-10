#[cfg(feature = "multithreaded")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::sync::{Arc, RwLock};

use crate::world::World;

pub type System = fn(&World);

#[derive(Default, Clone)]
pub struct Scheduler {
    #[cfg(feature = "multithreaded")]
    parallel_systems: Arc<RwLock<Vec<System>>>,
    systems: Arc<RwLock<Vec<System>>>,
}

impl Scheduler {
    #[cfg(feature = "multithreaded")]
    pub fn register_parallel(&self, system: System) {
        self.parallel_systems.write().unwrap().push(system)
    }

    pub fn register(&self, system: System) {
        self.systems.write().unwrap().push(system)
    }

    pub fn deregister(&self, system: System) {
        #[expect(unpredictable_function_pointer_comparisons)]
        let position = self
            .systems
            .read()
            .unwrap()
            .iter()
            .position(|&s| s == system);
        if let Some(pos) = position {
            let _ = self.systems.write().unwrap().remove(pos);
            #[cfg(feature = "multithreaded")]
            return;
        }
        #[cfg(feature = "multithreaded")]
        #[expect(unpredictable_function_pointer_comparisons)]
        let position = self
            .parallel_systems
            .read()
            .unwrap()
            .iter()
            .position(|&s| s == system);
        #[cfg(feature = "multithreaded")]
        if let Some(pos) = position {
            let _ = self.parallel_systems.write().unwrap().remove(pos);
        }
    }

    pub fn clear(&self) {
        self.systems.write().unwrap().clear();
    }

    pub fn run(&self, world: &mut World) {
        #[cfg(feature = "multithreaded")]
        let len = self.parallel_systems.read().unwrap().len();
        #[cfg(feature = "multithreaded")]
        let systems = &self.parallel_systems;
        #[cfg(feature = "multithreaded")]
        (0..len)
            .into_par_iter()
            .filter_map(|i| systems.read().unwrap().get(i).copied())
            .for_each(|sys| sys(world));

        let len = self.systems.read().unwrap().len();
        (0..len)
            .filter_map(|i| self.systems.read().unwrap().get(i).copied())
            .for_each(|sys| sys(world));
    }
}
