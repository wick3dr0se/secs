use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::RwLock,
};

use bimap::BiMap;

use crate::world::Entity;

pub struct SparseSet<C> {
    pub sparse: BiMap<Entity, usize>,
    pub dense: Vec<C>,
}

impl<C> SparseSet<C> {
    pub fn new(entity: Entity, component: C) -> Self {
        let mut sparse = BiMap::new();
        sparse.insert(entity, 0);

        Self {
            sparse,
            dense: vec![component],
        }
    }

    pub fn insert(&mut self, entity: Entity, component: C) {
        self.dense.push(component);
        self.sparse.insert(entity, self.dense.len() - 1);
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some((_entity, idx)) = self.sparse.remove_by_left(&entity) {
            let last = self.dense.len() - 1;

            if idx != last {
                self.dense.swap(idx, last);

                if let Some(&swapped_entity) = self.sparse.get_by_right(&last) {
                    self.sparse.insert(swapped_entity, idx);
                }
            }

            self.dense.pop();
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&usize> {
        self.sparse.get_by_left(&entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &C)> {
        self.sparse
            .iter()
            .map(|(&entity, &idx)| (entity, &self.dense[idx]))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.dense
            .iter_mut()
            .enumerate()
            .map(|(idx, component)| (*self.sparse.get_by_right(&idx).unwrap(), component))
    }
}

#[derive(Default)]
pub struct SparseSets {
    sets: HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>,
}

impl SparseSets {
    pub fn insert<C: 'static + Send + Sync>(&mut self, entity: Entity, component: C) {
        self.sets.insert(
            TypeId::of::<C>(),
            RwLock::new(Box::new(SparseSet::new(entity, component))),
        );
    }

    pub fn get<C: 'static>(&self) -> Option<&SparseSet<C>> {
        self.sets.get(&TypeId::of::<C>()).and_then(|set| {
            let guard = set.read().unwrap();

            unsafe {
                (guard.as_ref() as *const dyn Any)
                    .cast::<SparseSet<C>>()
                    .as_ref()
            }
        })
    }

    pub fn get_mut<C: 'static>(&self) -> Option<&mut SparseSet<C>> {
        self.sets.get(&TypeId::of::<C>()).and_then(|set| {
            let mut guard = set.write().unwrap();

            unsafe {
                (guard.as_mut() as *mut dyn Any)
                    .cast::<SparseSet<C>>()
                    .as_mut()
            }
        })
    }
}
