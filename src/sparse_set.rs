use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{any::TypeId, collections::HashMap};

use bimap::BiMap;

use crate::world::{Entity, SendSync};

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

    pub fn get(&self, entity: Entity) -> Option<usize> {
        self.sparse.get_by_left(&entity).copied()
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
    sets: HashMap<TypeId, RwLock<Box<dyn SendSync>>>,
}

impl SparseSets {
    pub fn insert<C: SendSync>(&mut self, entity: Entity, component: C) {
        self.sets.insert(
            TypeId::of::<C>(),
            RwLock::new(Box::new(SparseSet::new(entity, component))),
        );
    }

    pub fn get<C: 'static>(&self) -> Option<MappedRwLockReadGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_read() else {
            panic!(
                "Tried to access component `{}`, but it was already being written to",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockReadGuard::map(guard, |dynbox| unsafe {
            let dynthing: *const dyn SendSync = dynbox.as_ref();
            &*dynthing.cast::<SparseSet<C>>()
        }))
    }

    pub fn get_mut<C: 'static>(&self) -> Option<MappedRwLockWriteGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_write() else {
            panic!(
                "Tried to access component `{}` mutably, but it was already being written to or read from",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockWriteGuard::map(guard, |dynbox| unsafe {
            let dynthing: *mut dyn SendSync = dynbox.as_mut();
            &mut *dynthing.cast::<SparseSet<C>>()
        }))
    }
}
