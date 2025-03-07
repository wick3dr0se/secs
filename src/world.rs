use std::{
    any::{Any, TypeId},
    collections::HashMap,
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

use crate::{
    components::AttachComponents,
    query::Query,
    scheduler::{ExecutionMode, MutSystem, Scheduler, System},
    sparse_set::{SparseSet, SparseSets},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(NonZeroU64);

#[cfg(feature = "multithreaded")]
pub trait SendSync: Any + Send + Sync {}
#[cfg(not(feature = "multithreaded"))]
pub trait SendSync: Any {}
#[cfg(feature = "multithreaded")]
impl<T: ?Sized + Send + Sync + Any> SendSync for T {}
#[cfg(not(feature = "multithreaded"))]
impl<T: ?Sized + Any> SendSync for T {}

#[derive(Default)]
pub struct World {
    entities: AtomicU64,
    sparse_sets: SparseSets,
    scheduler: Scheduler,
    #[cfg(feature = "multithreaded")]
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    #[cfg(not(feature = "multithreaded"))]
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    #[track_caller]
    pub(crate) fn get_sparse_set<C: 'static>(&self) -> Option<MappedRwLockReadGuard<SparseSet<C>>> {
        self.sparse_sets.get::<C>()
    }

    #[track_caller]
    pub(crate) fn get_sparse_set_mut<C: 'static>(
        &self,
    ) -> Option<MappedRwLockWriteGuard<SparseSet<C>>> {
        self.sparse_sets.get_mut::<C>()
    }

    #[track_caller]
    pub(crate) fn attach_component<C: SendSync>(&self, entity: Entity, component: C) {
        if let Some(mut set) = self.sparse_sets.get_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(entity, component);
        }
    }

    #[track_caller]
    pub fn spawn<C: AttachComponents>(&self, components: C) -> Entity {
        let entity = self.entities.fetch_add(1, Ordering::Relaxed);
        let entity = Entity(NonZeroU64::new(entity + 1).unwrap());
        components.attach_to_entity(self, entity);
        entity
    }

    #[track_caller]
    pub fn despawn(&mut self, entity: Entity) {
        self.sparse_sets.remove(entity);
    }

    #[track_caller]
    pub fn attach<C: AttachComponents>(&self, entity: Entity, components: C) {
        components.attach_to_entity(self, entity);
    }

    #[track_caller]
    pub fn detach<C: 'static>(&self, entity: Entity) {
        if let Some(mut set) = self.sparse_sets.get_mut::<C>() {
            set.remove(entity)
        }
    }

    #[track_caller]
    pub fn get<C: 'static>(&self, entity: Entity) -> Option<MappedRwLockReadGuard<C>> {
        let set = self.sparse_sets.get::<C>()?;
        MappedRwLockReadGuard::try_map(set, |set| set.get(entity)).ok()
    }

    #[track_caller]
    pub fn get_mut<C: 'static>(&self, entity: Entity) -> Option<MappedRwLockWriteGuard<C>> {
        let set = self.sparse_sets.get_mut::<C>()?;
        MappedRwLockWriteGuard::try_map(set, |set| set.get_mut(entity)).ok()
    }

    #[track_caller]
    pub fn query<Q: Query>(
        &self,
        f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Q::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        Q::get_components(self, f)
    }

    pub fn query_retain<C: 'static>(&self, mut f: impl for<'a> FnMut(Entity, &'a mut C) -> bool) {
        let Some(mut set) = self.sparse_sets.get_mut::<C>() else {
            return;
        };
        let set = &mut *set;

        let mut idx = 0;
        while let Some(entity) = set.ids.get(idx).copied() {
            let retain = f(entity, &mut set.dense[idx]);
            if retain {
                idx += 1;
            } else {
                set.remove(entity);
            }
        }
    }

    pub fn add_resource<R: 'static + SendSync>(&mut self, res: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(res));
    }

    pub fn get_resource<R: 'static>(&self) -> Option<&R> {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|r| r.downcast_ref())
    }

    pub fn get_resource_mut<R: 'static>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|r| r.downcast_mut())
    }

    pub fn remove_resource<R: 'static>(&mut self) {
        self.resources.remove(&TypeId::of::<R>());
    }

    pub fn add_system(&mut self, system: System, mode: ExecutionMode) {
        self.scheduler.register(system, mode);
    }

    pub fn add_mut_system(&mut self, system: MutSystem) {
        self.scheduler.register_mut(system);
    }

    pub fn remove_system(&mut self, system: System) {
        self.scheduler.deregister(system);
    }

    pub fn remove_mut_system(&mut self, system: MutSystem) {
        self.scheduler.deregister_mut(system);
    }

    pub fn run_systems(&mut self) {
        // Shallow clone, everything is reference counted inside
        let scheduler = self.scheduler.clone();
        scheduler.run(self);
    }
}
