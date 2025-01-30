use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap};

use thunderdome::{Arena, Index};

use crate::{query::Query, sparse_set::SparseSet};

pub type Entity = Index;

#[derive(Default)]
pub struct World {
    entities: Arena<()>,
    sparse_sets: HashMap<TypeId, RefCell<Box<dyn Any>>>
}

impl World {
    pub fn get_sparse_set<C: 'static>(&self) -> Option<&SparseSet<C>> {
        self.sparse_sets.get(&TypeId::of::<C>()).and_then(|set| unsafe {
            (set.borrow().as_ref() as *const dyn Any).cast::<SparseSet<C>>().as_ref()
        })
    }    

    pub fn get_sparse_set_mut<C: 'static>(&self) -> Option<&mut SparseSet<C>> {
        self.sparse_sets.get(&TypeId::of::<C>()).and_then(|set| unsafe {
            (set.borrow_mut().as_mut() as *mut dyn Any).cast::<SparseSet<C>>().as_mut()
        })
    }

    pub fn spawn(&mut self) -> Entity { self.entities.insert(()) }

    pub fn attach<C: 'static>(&mut self, entity: Entity, component: C) {
        if let Some(set) = self.get_sparse_set_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(TypeId::of::<C>(), RefCell::new(Box::new(SparseSet::new(entity, component))));
        }
    }

    pub fn detach<C: 'static>(&mut self, entity: Entity) {
        self.get_sparse_set_mut::<C>().map(|set| set.remove(entity));
    }

    pub fn query<'a, Q: Query<'a>>(&'a self) -> impl Iterator<Item = (thunderdome::Index, Q)> + 'a {
        Q::get_components(self).into_iter().flatten()
    }
}