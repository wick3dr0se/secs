use crate::world::World;
use std::cell::{Cell, RefCell};

/// A unique ID for a specific system generated when
/// the system was [registered](Scheduler::register).
/// Can be used to [deregister](Scheduler::deregister) the system later.
#[derive(Copy, Clone)]
pub struct SysId(u64);

pub type System<'a, RES> = (SysId, Option<Box<dyn FnMut(&World, &mut RES) + 'a>>);

pub struct Scheduler<'a, RES> {
    next_id: Cell<u64>,
    systems: RefCell<Vec<System<'a, RES>>>,
}

impl<RES> Default for Scheduler<'_, RES> {
    fn default() -> Self {
        Self {
            next_id: Default::default(),
            systems: Default::default(),
        }
    }
}

impl<'a, RES> Scheduler<'a, RES> {
    /// Add a system that will run after all systems that were added before it.
    pub fn register(&self, system: impl FnMut(&World, &mut RES) + 'a) -> SysId {
        let id = SysId(self.next_id.get());
        self.next_id.set(id.0 + 1);
        self.systems.borrow_mut().push((id, Some(Box::new(system))));
        id
    }

    /// Remove a previously inserted system. Will silently do nothing if the system was already removed.
    pub fn deregister(&self, system: SysId) {
        let position = self
            .systems
            .borrow()
            .iter()
            .position(|(id, _)| id.0 == system.0);
        if let Some(pos) = position {
            let _ = self.systems.borrow_mut().remove(pos);
        }
    }

    /// Run all systems once.
    pub fn run(&self, world: &World, res: &mut RES) {
        let len = self.systems.borrow().len();
        for i in 0..len {
            let mut guard = self.systems.borrow_mut();
            let Some((_, sys)) = guard.get_mut(i) else {
                break;
            };
            let mut sys = sys.take().unwrap();
            drop(guard);
            sys(world, res);
            let mut guard = self.systems.borrow_mut();
            let Some((_, entry)) = guard.get_mut(i) else {
                break;
            };
            *entry = Some(sys);
        }
    }
}
