use std::{any::{Any, TypeId}, collections::HashMap, sync::RwLock};

use thunderdome::{Arena, Index};

use crate::{query::Query, scheduler::{ExecutionMode, Scheduler, System}, sparse_set::SparseSet};

pub type Entity = Index;

#[derive(Default)]
pub struct World {
    entities: Arena<()>,
    sparse_sets: HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>,
    scheduler: Scheduler
}

impl World {
    pub fn get_sparse_set<C: 'static>(&self) -> Option<&SparseSet<C>> {
        self.sparse_sets.get(&TypeId::of::<C>()).and_then(|set| {
            let guard = set.read().unwrap();

            unsafe { (guard.as_ref() as *const dyn Any).cast::<SparseSet<C>>().as_ref() }
        })
    }

    pub fn get_sparse_set_mut<C: 'static>(&self) -> Option<&mut SparseSet<C>> {
        self.sparse_sets.get(&TypeId::of::<C>()).and_then(|set| {
            let mut guard = set.write().unwrap();

            unsafe {
                (guard.as_mut() as *mut dyn Any).cast::<SparseSet<C>>().as_mut()
            }
        })
    }

    pub fn spawn(&mut self) -> Entity { self.entities.insert(()) }

    pub fn attach<C: 'static + Send + Sync>(&mut self, entity: Entity, component: C) {
        if let Some(set) = self.get_sparse_set_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(TypeId::of::<C>(), RwLock::new(Box::new(SparseSet::new(entity, component))));
        }
    }

    pub fn detach<C: 'static>(&mut self, entity: Entity) {
        self.get_sparse_set_mut::<C>().map(|set| set.remove(entity));
    }

    pub fn query<'a, Q: Query<'a>>(&'a self) -> impl Iterator<Item = (thunderdome::Index, Q)> + 'a {
        Q::get_components(self).into_iter().flatten()
    }

    pub fn add_system(&mut self, system: System, mode: ExecutionMode) {
        self.scheduler.register(system, mode);
    }

    pub fn run_systems(&self) {
        self.scheduler.run(self);
    }
}