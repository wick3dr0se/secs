use crate::world::World;
use std::cell::{Cell, RefCell};

/// A unique ID for a specific system generated when
/// the system was [registered](Scheduler::register).
/// Can be used to [deregister](Scheduler::deregister) the system later.
#[derive(Copy, Clone)]
pub struct SysId(u64);

pub trait SystemFn: FnMut(&World) + 'static {}

impl<T: FnMut(&World) + 'static> SystemFn for T {}

pub type System = (SysId, Option<Box<dyn SystemFn>>);

#[derive(Default)]
pub struct Scheduler {
    next_id: Cell<u64>,
    systems: RefCell<Vec<System>>,
}

impl Scheduler {
    pub fn register(&self, system: impl SystemFn) -> SysId {
        let id = SysId(self.next_id.get());
        self.next_id.set(id.0 + 1);
        self.systems.borrow_mut().push((id, Some(Box::new(system))));
        id
    }

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

    pub fn run(&self, world: &World) {
        let len = self.systems.borrow().len();
        for i in 0..len {
            let mut guard = self.systems.borrow_mut();
            let Some((_, sys)) = guard.get_mut(i) else {
                break;
            };
            let mut sys = sys.take().unwrap();
            drop(guard);
            sys(world);
            let mut guard = self.systems.borrow_mut();
            let Some((_, entry)) = guard.get_mut(i) else {
                break;
            };
            *entry = Some(sys);
        }
    }
}
