use std::{any::{Any, TypeId}, collections::HashMap};

use thunderdome::{Arena, Index};

use crate::{components::AttachComponents, query::Query, scheduler::{ExecutionMode, Scheduler, System}, sparse_set::{SparseSet, SparseSets}};

pub type Entity = Index;

#[derive(Default)]
pub struct World {
    entities: Arena<()>,
    sparse_sets: SparseSets,
    scheduler: Scheduler,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

impl World {
    pub(crate) fn attach_component<C: 'static + Send + Sync>(&mut self, entity: Entity, component: C) {
        if let Some(set) = self.sparse_sets.get_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(entity, component);
        }
    }

    pub fn spawn<C: AttachComponents>(&mut self, components: C) -> Entity {
        let entity = self.entities.insert(());
        components.attach_to_entity(self, entity);
        entity
    }

    pub fn attach<C: AttachComponents>(&mut self, entity: Entity, components: C) {
        components.attach_to_entity(self, entity);
    }

    pub fn detach<C: 'static>(&mut self, entity: Entity) {
        self.sparse_sets.get_mut::<C>().map(|set| set.remove(entity));
    }

    pub fn query<'a, Q: Query<'a>>(&'a self) -> impl Iterator<Item = (thunderdome::Index, Q)> + 'a {
        Q::get_components(self).into_iter().flatten()
    }

    pub fn add_resource<R: 'static + Send + Sync>(&mut self, res: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(res));
    }

    pub fn get_resource<R: 'static>(&self) -> Option<&R> {
        self.resources.get(&TypeId::of::<R>()).and_then(|r| r.downcast_ref())
    }

    pub fn get_resource_mut<R: 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut(&TypeId::of::<R>()).and_then(|r| r.downcast_mut())
    }

    pub fn remove_resource<R: 'static>(&mut self) {
        self.resources.remove(&TypeId::of::<R>());
    }

    pub fn add_system(&mut self, system: System, mode: ExecutionMode) {
        self.scheduler.register(system, mode);
    }

    pub fn remove_system(&mut self, system: System) {
        self.scheduler.deregister(system);
    }

    pub fn run_systems(&self) {
        self.scheduler.run(self);
    }
}

pub trait WorldQuery {
    fn get_sparse_set<C: 'static>(&self) -> Option<&SparseSet<C>>;
    fn get_sparse_set_mut<C: 'static>(&self) -> Option<&mut SparseSet<C>>;
}

impl WorldQuery for World {
    fn get_sparse_set<C: 'static>(&self) -> Option<&SparseSet<C>> {
        self.sparse_sets.get::<C>()
    }

    fn get_sparse_set_mut<C: 'static>(&self) -> Option<&mut SparseSet<C>> {
        self.sparse_sets.get_mut::<C>()
    }
}