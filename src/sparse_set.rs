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
}

pub struct SparseSetIter<'a, C> {
    iter: bimap::hash::Iter<'a, Entity, usize>,
    dense: &'a [C],
}

impl<'a, C> Iterator for SparseSetIter<'a, C> {
    type Item = (Entity, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(&entity, &idx)| (entity, &self.dense[idx]))
    }
}

impl<'a, C> IntoIterator for &'a SparseSet<C> {
    type Item = (Entity, &'a C);

    type IntoIter = SparseSetIter<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        SparseSetIter {
            iter: self.sparse.iter(),
            dense: &self.dense,
        }
    }
}

pub struct SparseSetIterMut<'a, C> {
    iter: bimap::hash::Iter<'a, Entity, usize>,
    dense: &'a mut [C],
}

impl<'a, C> Iterator for SparseSetIterMut<'a, C> {
    type Item = (Entity, &'a mut C);

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: We know for a fact that no two entries in `iter` have the same `usize` value,
        // so we are able to ensure mutual exclusion.
        let dense: &'a mut [C] =
            unsafe { std::slice::from_raw_parts_mut(self.dense.as_mut_ptr(), self.dense.len()) };
        self.iter
            .next()
            .map(move |(&entity, &idx)| (entity, &mut dense[idx]))
    }
}

impl<'a, C> IntoIterator for &'a mut SparseSet<C> {
    type Item = (Entity, &'a mut C);

    type IntoIter = SparseSetIterMut<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        SparseSetIterMut {
            iter: self.sparse.iter(),
            dense: &mut self.dense,
        }
    }
}

trait Set: SendSync {
    fn remove(&mut self, entity: Entity);
}

impl<C: SendSync> Set for SparseSet<C> {
    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }
}

#[derive(Default)]
pub struct SparseSets {
    sets: HashMap<TypeId, RwLock<Box<dyn Set>>>,
}

impl SparseSets {
    pub fn insert<C: SendSync>(&mut self, entity: Entity, component: C) {
        self.sets.insert(
            TypeId::of::<C>(),
            RwLock::new(Box::new(SparseSet::new(entity, component))),
        );
    }

    pub fn remove(&mut self, entity: Entity) {
        for set in self.sets.values() {
            let Some(mut guard) = set.try_write() else {
                panic!(
                    "Tried to access component mutably, but it is already being read or written to",
                )
            };
            guard.remove(entity);
        }
    }

    #[track_caller]
    pub fn get<C: 'static>(&self) -> Option<MappedRwLockReadGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_read() else {
            panic!(
                "Tried to access component `{}`, but it was already being written to",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockReadGuard::map(guard, |dynbox| unsafe {
            let dynthing: *const dyn Set = dynbox.as_ref();
            &*dynthing.cast::<SparseSet<C>>()
        }))
    }

    #[track_caller]
    pub fn get_mut<C: 'static>(&self) -> Option<MappedRwLockWriteGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_write() else {
            panic!(
                "Tried to access component `{}` mutably, but it was already being written to or read from",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockWriteGuard::map(guard, |dynbox| unsafe {
            let dynthing: *mut dyn Set = dynbox.as_mut();
            &mut *dynthing.cast::<SparseSet<C>>()
        }))
    }
}
