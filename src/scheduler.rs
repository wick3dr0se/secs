#[cfg(feature = "multithreaded")]
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

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

pub trait SystemFn: FnMut(&World) + SendSync {}

impl<T: FnMut(&World) + SendSync> SystemFn for T {}

pub type System = (SysId, Option<Box<dyn SystemFn>>);

#[cfg(feature = "multithreaded")]
pub trait ParallelSystemFn: Fn(&World) + SendSync {}

#[cfg(feature = "multithreaded")]
impl<T: Fn(&World) + SendSync> ParallelSystemFn for T {}

#[cfg(feature = "multithreaded")]
pub type ParallelSystem = (SysId, Arc<dyn ParallelSystemFn>);

#[derive(Default)]
pub struct Scheduler {
    next_id: AtomicU64,
    #[cfg(feature = "multithreaded")]
    parallel_systems: RwLock<Vec<ParallelSystem>>,
    systems: RwLock<Vec<System>>,
}

impl Scheduler {
    #[cfg(feature = "multithreaded")]
    pub fn register_parallel(&self, system: impl ParallelSystemFn) -> SysId {
        let id = SysId(self.next_id.fetch_add(1, Ordering::Relaxed));
        self.parallel_systems.write().push((id, Arc::new(system)));
        id
    }

    pub fn register(&self, system: impl SystemFn) -> SysId {
        let id = SysId(self.next_id.fetch_add(1, Ordering::Relaxed));
        self.systems.write().push((id, Some(Box::new(system))));
        id
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
        for i in 0..len {
            let mut guard = self.systems.write();
            let Some((_, sys)) = guard.get_mut(i) else {
                break;
            };
            let mut sys = sys.take().unwrap();
            drop(guard);
            sys(world);
            let mut guard = self.systems.write();
            let Some((_, entry)) = guard.get_mut(i) else {
                break;
            };
            *entry = Some(sys);
        }
    }
}
