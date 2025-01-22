use std::{any::{Any, TypeId}, collections::HashMap};

use bimap::BiMap; 
use thunderdome::{Arena, Index};

pub type Entity = Index;

pub trait Component: 'static {}

pub struct SpareSet<C> {
    sparse: BiMap<Entity, usize>,
    dense: Vec<C>
}

impl<C> SpareSet<C> {
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
    sparesets: HashMap<TypeId, Box<dyn Any>>
}

impl World {
    fn get_sparseset<C: Component>(&self) -> Option<&SpareSet<C>> {
        self.sparesets.get(&TypeId::of::<C>())
            .and_then(|b| b.downcast_ref())
    }

    fn get_sparseset_mut<C: Component>(&mut self) -> Option<&mut SpareSet<C>> {
        self.sparesets.get_mut(&TypeId::of::<C>())
            .and_then(|b| b.downcast_mut())
    }

    pub fn spawn(&mut self) -> Entity { self.entities.insert(()) }

    pub fn attach<C: Component>(&mut self, entity: Entity, component: C) {
        if let Some(set) = self.get_sparseset_mut::<C>() {
            set.insert(entity, component);
        } else {
            self.sparesets.insert(TypeId::of::<C>(), Box::new(SpareSet::new(entity, component)));
        }
    }

    pub fn detach<C: Component>(&mut self, entity: Entity) {
        self.get_sparseset_mut::<C>().map(|set| set.remove(entity));
    }

    pub fn query<'a, Q: Query<'a>>(&'a self) -> impl Iterator<Item = (Entity, Q)> + 'a {
        Q::get_components(self).unwrap()
    }
    
    pub fn query_mut<'a, Q: QueryMut<'a>>(&'a mut self) -> impl Iterator<Item = (Entity, Q)> + 'a {
        Q::get_components_mut(self).unwrap()
    }
}

pub trait Query<'a> {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a>; 
}

pub trait QueryMut<'a> {
    fn get_components_mut(world: &'a mut World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a>;
}

impl<'a, C: Component> Query<'a> for (&'a C,) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparseset::<C>().and_then(|set| {
            Some(set.iter().map(|(entity, component)| (entity, (component,))))
        })
    }
}

impl<'a, C: Component> QueryMut<'a> for (&'a mut C,) {
    fn get_components_mut(world: &'a mut World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparseset_mut::<C>().and_then(|set| {
            Some(set.iter_mut().map(|(entity, component)| (entity, (component,))))
        })
    }
}

impl<'a, C1: Component, C2: Component> Query<'a> for (&'a C1, &'a C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparseset::<C1>()?;
        let s2 = world.get_sparseset::<C2>()?;

        Some(s1.iter().filter_map(|(entity, c1)| {
            s2.sparse.get_by_left(&entity).map(|_| {
                let c2 = &s2.dense[*s2.sparse.get_by_left(&entity).unwrap()];
                
                (entity, (c1, c2))
            })
        }))
    }
}