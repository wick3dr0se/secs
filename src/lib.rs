use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap};

use bimap::BiMap; 
use thunderdome::{Arena, Index};

pub type Entity = Index;

pub trait Component: 'static {}

pub struct SparseSet<C> {
    sparse: BiMap<Entity, usize>,
    dense: Vec<C>
}

impl<C> SparseSet<C> {
    fn new(entity: Entity, component: C) -> Self {
        let mut sparse = BiMap::new();
        sparse.insert(entity, 0);

        Self {
            sparse,
            dense: vec![component]
        }
    }

    fn insert(&mut self, entity: Entity, component: C) {
        self.dense.push(component);
        self.sparse.insert(entity, self.dense.len() - 1);
    }

    fn remove(&mut self, entity: Entity) {
        if let Some((_entity, idx)) = self.sparse.remove_by_left(&entity) {
            let last = self.dense.len() - 1;

            if idx != last {
                self.dense.swap(idx, last);
                
                let swapped_entity = self.sparse.iter()
                    .find(|(_, &i)| i == last)
                    .map(|(entity, _)| *entity)
                    .unwrap();

                self.sparse.insert(swapped_entity, idx);
            }

            self.dense.pop();
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Entity, &C)> {
        self.sparse.iter().map(|(&entity, &idx)| (entity, &self.dense[idx]))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.dense.iter_mut().enumerate().map(|(idx, component)| {
            (*self.sparse.get_by_right(&idx).unwrap(), component)
        })
    }
}

#[derive(Default)]
pub struct World {
    entities: Arena<()>,
    sparse_sets: HashMap<TypeId, RefCell<Box<dyn Any>>>
}

impl World {
    fn get_sparse_set<C: Component>(&self) -> Option<&SparseSet<C>> {
        self.sparse_sets.get(&TypeId::of::<C>()).and_then(|set| unsafe {
            (set.borrow().as_ref() as *const dyn Any).cast::<SparseSet<C>>().as_ref()
        })
    }    

    fn get_sparse_set_mut<C: Component>(&self) -> Option<&mut SparseSet<C>> {
        self.sparse_sets.get(&std::any::TypeId::of::<C>()).and_then(|set| unsafe {
            (set.borrow_mut().as_mut() as *mut dyn Any).cast::<SparseSet<C>>().as_mut()
        })
    }

    pub fn spawn(&mut self) -> Entity { self.entities.insert(()) }

    pub fn attach<C: Component>(&mut self, entity: Entity, component: C) {
        if let Some(set) = self.get_sparse_set_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparse_sets.insert(TypeId::of::<C>(), RefCell::new(Box::new(SparseSet::new(entity, component))));
        }
    }

    pub fn detach<C: Component>(&mut self, entity: Entity) {
        self.get_sparse_set_mut::<C>().map(|set| set.remove(entity));
    }

    pub fn query<'a, Q: Query<'a>>(&'a self) -> impl Iterator<Item = (Entity, Q)> + 'a {
        Q::get_components(self).unwrap()
    }
}

pub trait Query<'a> {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a>; 
}

impl<'a, C: Component> Query<'a> for (&'a C,) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparse_set::<C>().and_then(|set| {
            Some(set.iter().map(|(entity, component)| (entity, (component,))))
        })
    }
}

impl<'a, C: Component> Query<'a> for (&'a mut C,) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparse_set_mut::<C>().and_then(|set| {
            Some(set.iter_mut().map(|(entity, component)| (entity, (component,))))
        })
    }
}

impl<'a, C1: Component, C2: Component> Query<'a> for (&'a C1, &'a C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;

        Some(s1.iter().filter_map(|(entity, c1)| {
            s2.sparse.get_by_left(&entity).map(|_| {
                let c2 = &s2.dense[*s2.sparse.get_by_left(&entity).unwrap()];
                
                (entity, (c1, c2))
            })
        }))
    }
}