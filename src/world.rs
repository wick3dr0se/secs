use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap},
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

use crate::{
    components::AttachComponents,
    query::Query,
    scheduler::{MutSystem, Scheduler, System},
    sparse_set::{SparseSet, SparseSets},
};

/// An opaque id for an entity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(NonZeroU64);

impl Entity {
    /// Returns the unique numeric identifier of this entity.
    pub fn id(&self) -> u64 {
        self.0.get()
    }
}

#[cfg(feature = "multithreaded")]
pub trait SendSync: Any + Send + Sync {}
#[cfg(not(feature = "multithreaded"))]
pub trait SendSync: Any {}
#[cfg(feature = "multithreaded")]
impl<T: ?Sized + Send + Sync + Any> SendSync for T {}
#[cfg(not(feature = "multithreaded"))]
impl<T: ?Sized + Any> SendSync for T {}

/// The main entry point to this [crate].
///
/// Invoke [Self::spawn] to add entities and [Self::query] to process them.
#[derive(Default)]
pub struct World {
    entities: AtomicU64,
    dead_entities: BTreeSet<Entity>,
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
        assert!(
            !self.dead_entities.contains(&entity),
            "Attaching `{}` to despawned entity",
            std::any::type_name::<C>(),
        );
        if let Some(mut set) = self.sparse_sets.get_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(entity, component);
        }
    }

    /// Create a new entity and immediately initialize it with the given components.
    ///
    /// ```rust
    /// # use secs::prelude::*;
    /// # let world = World::default();
    /// world.spawn(("player", 42));
    /// world.spawn(("animal", 12, 5.3));
    /// ```
    #[track_caller]
    pub fn spawn<C: AttachComponents>(&self, components: C) -> Entity {
        let entity = self.entities.fetch_add(1, Ordering::Relaxed);
        let entity = Entity(NonZeroU64::new(entity + 1).unwrap());
        components.attach_to_entity(self, entity);
        entity
    }

    /// Detach all components from an entity and drop them.
    /// If you to extract specific components, call [Self::detach] first.
    #[track_caller]
    pub fn detach_all(&mut self, entity: Entity) {
        assert!(
            !self.dead_entities.contains(&entity),
            "Removing an already removed entity"
        );
        self.sparse_sets.remove(entity);
    }

    /// Destroy an entity and all its components. Future attempts to use this entity in any way will panic.
    #[track_caller]
    pub fn despawn(&mut self, entity: Entity) {
        self.detach_all(entity);
        self.dead_entities.insert(entity);
    }

    /// Attach multiple components to an entity at once.
    #[track_caller]
    pub fn attach<C: AttachComponents>(&self, entity: Entity, components: C) {
        components.attach_to_entity(self, entity);
    }

    /// Detach a component and return it if the entity had that component.
    #[track_caller]
    pub fn detach<C: 'static>(&self, entity: Entity) -> Option<C> {
        assert!(
            !self.dead_entities.contains(&entity),
            "Detaching `{}` from despawned entity",
            std::any::type_name::<C>(),
        );
        let mut set = self.sparse_sets.get_mut::<C>()?;
        set.remove(entity)
    }

    /// Immutable access to an entity's component.
    ///
    /// # Panics
    ///
    /// This will panic if the component is already used mutably either by a [Self::query] or [Self::get_mut].
    #[track_caller]
    pub fn get<C: 'static>(&self, entity: Entity) -> Option<MappedRwLockReadGuard<C>> {
        assert!(
            !self.dead_entities.contains(&entity),
            "Getting `{}` from despawned entity",
            std::any::type_name::<C>(),
        );
        let set = self.sparse_sets.get::<C>()?;
        MappedRwLockReadGuard::try_map(set, |set| set.get(entity)).ok()
    }

    /// Mutable access to an entity's component.
    ///
    /// # Panics
    ///
    /// This will panic if the component is already used either by a [Self::query], [Self::get_mut], or [Self::get].
    #[track_caller]
    pub fn get_mut<C: 'static>(&self, entity: Entity) -> Option<MappedRwLockWriteGuard<C>> {
        assert!(
            !self.dead_entities.contains(&entity),
            "Getting `{}` from despawned entity",
            std::any::type_name::<C>(),
        );
        let set = self.sparse_sets.get_mut::<C>()?;
        MappedRwLockWriteGuard::try_map(set, |set| set.get_mut(entity)).ok()
    }

    /// Invokes a closure for every entity that has the given components.
    ///
    /// The first component is the one that will be iterated over, so sorting the components to have
    /// the rarest one first is more performant than iterating over a common component and getting
    /// the entities discarded because a later component does not exist for it.
    ///
    /// ```rust
    /// # use secs::prelude::*;
    /// # let world = World::default();
    /// world.query::<(&String, &u32)>(|entity_id, (s, u)| {
    ///     println!("{s}: {u}");
    /// });
    /// ```
    #[track_caller]
    pub fn query<Q: Query>(
        &self,
        f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Q::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        Q::get_components(self, f)
    }

    /// Same as [Self::query], but only for one component and returns a boolean.
    /// If the boolean is `false` the component will be dropped.
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

    /// Register a global resource that can be accessed via [Self::get_resource] or [Self::get_resource_mut].
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

    /// Remove a global resource and get it back in an owned manner.
    pub fn remove_resource<R: 'static>(&mut self) -> Option<Box<R>> {
        Some(
            self.resources
                .remove(&TypeId::of::<R>())?
                .downcast()
                .unwrap(),
        )
    }

    /// Add a system that will run in parallel on threads with all
    /// other parallel systems.
    #[cfg(feature = "multithreaded")]
    pub fn add_parallel_system(&mut self, system: System) {
        self.scheduler.register_parallel(system);
    }

    /// Add a system that will run after all systems that were added before it.
    pub fn add_system(&mut self, system: System) {
        self.scheduler.register(system);
    }

    /// Same as [Self::add_system], but the system has mutable access to the [World].
    pub fn add_mut_system(&mut self, system: MutSystem) {
        self.scheduler.register_mut(system);
    }

    /// Remove a system. Note that due to how compilers work this may not
    /// work if the system is declared in another crate.
    pub fn remove_system(&mut self, system: System) {
        self.scheduler.deregister(system);
    }

    /// Same as [Self::remove_system].
    pub fn remove_mut_system(&mut self, system: MutSystem) {
        self.scheduler.deregister_mut(system);
    }

    /// Run all systems once.
    pub fn run_systems(&mut self) {
        // Shallow clone, everything is reference counted inside
        let scheduler = self.scheduler.clone();
        scheduler.run(self);
    }

    /// Clear the world by removing all entities, components, systems and resources
    pub fn clear(&mut self) {
        self.entities = AtomicU64::new(0);
        self.dead_entities.clear();
        self.sparse_sets.clear();
        self.resources.clear();
        self.scheduler.clear();
    }
}
