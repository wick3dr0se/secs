use elsa::FrozenVec;
use std::{
    any::{Any, TypeId, type_name},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use crate::world::Entity;

pub struct SparseSet<C> {
    pub sparse: HashMap<Entity, usize>,
    pub dense: Vec<C>,
    pub ids: Vec<Entity>,
}

impl<C> SparseSet<C> {
    pub fn new(entity: Entity, component: C) -> Self {
        let mut sparse = HashMap::new();
        sparse.insert(entity, 0);

        Self {
            sparse,
            dense: vec![component],
            ids: vec![entity],
        }
    }

    pub fn insert(&mut self, entity: Entity, component: C) {
        self.sparse.insert(entity, self.dense.len());
        self.dense.push(component);
        self.ids.push(entity);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<C> {
        let idx = self.sparse.remove(&entity)?;
        let last = self.dense.len() - 1;

        if idx == last {
            self.ids.pop();
            self.dense.pop()
        } else {
            self.ids.swap_remove(idx);

            let _prev = self.sparse.insert(self.ids[idx], idx);
            debug_assert_eq!(_prev, Some(last));
            Some(self.dense.swap_remove(idx))
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        let &id = self.sparse.get(&entity)?;
        Some(&self.dense[id])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        let &id = self.sparse.get(&entity)?;
        Some(&mut self.dense[id])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &C)> {
        self.ids.iter().copied().zip(self.dense.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.ids.iter().copied().zip(self.dense.iter_mut())
    }

    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.ids.clear();
    }
}

trait Set: Any {
    fn debug(&mut self, entity: Entity) -> Option<&'static str>;

    fn remove(&mut self, entity: Entity);
}

impl<C: Any> Set for SparseSet<C> {
    fn debug(&mut self, entity: Entity) -> Option<&'static str> {
        self.get(entity).map(|_| type_name::<C>())
    }

    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }
}

#[derive(Default)]
pub struct SparseSets {
    set_access: RefCell<HashMap<TypeId, usize>>,
    sets: FrozenVec<Box<RefCell<dyn Set>>>,
}

impl SparseSets {
    pub fn insert<C: Any>(&self, entity: Entity, component: C) {
        let component = Box::new(RefCell::new(SparseSet::new(entity, component)));
        let n = self.sets.len();
        self.sets.push(component);
        assert_eq!(
            self.set_access.borrow_mut().insert(TypeId::of::<C>(), n),
            None
        );
    }

    #[track_caller]
    pub fn debug(&self, entity: Entity) -> String {
        #[cfg(any(debug_assertions, feature = "track_dead_entities"))]
        let mut component = String::new();

        for set in self.sets.iter() {
            let Ok(mut guard) = set.try_borrow_mut() else {
                panic!(
                    "Tried to access component mutably, but it is already being read or written to",
                )
            };

            #[cfg(any(debug_assertions, feature = "track_dead_entities"))]
            if let Some(c) = guard.debug(entity) {
                component.push_str(c);
                component.push_str(", ");
            }
            #[cfg(not(any(debug_assertions, feature = "track_dead_entities")))]
            guard.remove(entity);
        }
        #[cfg(any(debug_assertions, feature = "track_dead_entities"))]
        component
    }

    #[track_caller]
    pub fn remove(&self, entity: Entity) {
        for set in self.sets.iter() {
            let Ok(mut guard) = set.try_borrow_mut() else {
                panic!(
                    "Tried to access component mutably, but it is already being read or written to",
                )
            };

            guard.remove(entity);
        }
    }

    #[track_caller]
    pub fn get<C: 'static>(&self) -> Option<Ref<SparseSet<C>>> {
        let i = *self.set_access.borrow().get(&TypeId::of::<C>())?;
        let set = &self.sets.get(i).unwrap();
        let Ok(guard) = set.try_borrow() else {
            panic!(
                "Tried to access component `{}`, but it was already being written to",
                type_name::<C>()
            )
        };
        Some(Ref::map(guard, |dynbox| {
            (dynbox as &dyn Any).downcast_ref::<SparseSet<C>>().unwrap()
        }))
    }

    #[track_caller]
    pub fn get_mut<C: 'static>(&self) -> Option<RefMut<SparseSet<C>>> {
        let i = *self.set_access.borrow().get(&TypeId::of::<C>())?;
        let set = &self.sets.get(i).unwrap();
        let Ok(guard) = set.try_borrow_mut() else {
            panic!(
                "Tried to access component `{}` mutably, but it was already being written to or read from",
                type_name::<C>()
            )
        };
        Some(RefMut::map(guard, |dynbox| {
            (dynbox as &mut dyn Any)
                .downcast_mut::<SparseSet<C>>()
                .unwrap()
        }))
    }
}
