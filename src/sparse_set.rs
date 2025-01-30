use bimap::BiMap;

use crate::world::Entity;

pub struct SparseSet<C> {
    pub sparse: BiMap<Entity, usize>,
    pub dense: Vec<C>
}

impl<C> SparseSet<C> {
    pub fn new(entity: Entity, component: C) -> Self {
        let mut sparse = BiMap::new();
        sparse.insert(entity, 0);

        Self {
            sparse,
            dense: vec![component]
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
        self.sparse.iter().map(|(&entity, &idx)| (entity, &self.dense[idx]))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.dense.iter_mut().enumerate().map(|(idx, component)| {
            (*self.sparse.get_by_right(&idx).unwrap(), component)
        })
    }
}