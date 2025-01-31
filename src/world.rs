use thunderdome::{Arena, Index};

use crate::{query::Query, scheduler::{ExecutionMode, Scheduler, System}, sparse_set::{SparseSet, SparseSets}};

pub type Entity = Index;

pub trait AttachComponents {
    fn attach_to_entity(self, world: &mut World, entity: Entity);
}

impl<C1: 'static + Send + Sync> AttachComponents for (C1,) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
    }
}

impl<C1: 'static + Send + Sync, C2: 'static + Send + Sync> AttachComponents for (C1, C2) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
    }
}

impl<
    C1: 'static + Send + Sync,
    C2: 'static + Send + Sync,
    C3: 'static + Send + Sync
> AttachComponents for (C1, C2, C3) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
    }
}

impl<
    C1: 'static + Send + Sync,
    C2: 'static + Send + Sync,
    C3: 'static + Send + Sync,
    C4: 'static + Send + Sync
> AttachComponents for (C1, C2, C3, C4) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
    }
}

impl<
    C1: 'static + Send + Sync,
    C2: 'static + Send + Sync,
    C3: 'static + Send + Sync,
    C4: 'static + Send + Sync,
    C5: 'static + Send + Sync
> AttachComponents for (C1, C2, C3, C4, C5) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
        world.attach_component(entity, self.4);
    }
}

#[derive(Default)]
pub struct World {
    entities: Arena<()>,
    sparse_sets: SparseSets,
    scheduler: Scheduler
}

impl World {
    fn attach_component<C: 'static + Send + Sync>(&mut self, entity: Entity, component: C) {
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

    pub fn add_system(&mut self, system: System, mode: ExecutionMode) {
        self.scheduler.register(system, mode);
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