use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::world::SendSync;
use parking_lot::RwLock;
#[cfg(feature = "multithreaded")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::world::World;

/// A unique ID for a specific system generated when
/// the system was [registered](Scheduler::register).
/// Can be used to [deregister](Scheduler::deregister) the system later.
#[derive(Copy, Clone)]
pub struct SysId(u64);

pub trait SystemFn: Fn(&World) + SendSync {}

impl<T: Fn(&World) + SendSync> SystemFn for T {}

pub type System = (SysId, Arc<dyn SystemFn>);

#[derive(Default)]
pub struct Scheduler {
    next_id: AtomicU64,
    #[cfg(feature = "multithreaded")]
    parallel_systems: RwLock<Vec<System>>,
    systems: RwLock<Vec<System>>,
}

impl Scheduler {
    fn add_to(&self, systems: &RwLock<Vec<System>>, system: impl SystemFn) -> SysId {
        let id = SysId(self.next_id.fetch_add(1, Ordering::Relaxed));
        systems.write().push((id, Arc::new(system)));
        id
    }

    #[cfg(feature = "multithreaded")]
    pub fn register_parallel(&self, system: impl SystemFn) -> SysId {
        self.add_to(&self.parallel_systems, system)
    }

    pub fn register(&self, system: impl SystemFn) -> SysId {
        self.add_to(&self.systems, system)
    }

    pub fn deregister(&self, system: SysId) {
        let position = self
            .systems
            .read()
            .iter()
            .position(|(id, _)| id.0 == system.0);
        if let Some(pos) = position {
            let _ = self.systems.write().remove(pos);
            #[cfg(feature = "multithreaded")]
            return;
        }
        #[cfg(feature = "multithreaded")]
        let position = self
            .parallel_systems
            .read()
            .iter()
            .position(|(id, _)| id.0 == system.0);
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
            .filter_map(|i| Some(systems.read().get(i)?.1.clone()))
            .for_each(|sys| sys(world));

        let len = self.systems.read().len();
        (0..len)
            .filter_map(|i| Some(self.systems.read().get(i)?.1.clone()))
            .for_each(|sys| sys(world));
    }
}
