use crate::world::{Entity, World, WorldQuery};

pub trait Query<'a> {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a>; 
}

impl<'a, C: 'static> Query<'a> for (&'a C,) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparse_set::<C>().map(|set| {
            set.iter().map(|(entity, component)| (entity, (component,)))
        })
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a C1, &'a C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;

        Some(s1.iter().filter_map(|(entity, c1)| {
            s2.get(entity).map(|_| {
                let idx2 = s2.get(entity).unwrap();
                let c2 = &s2.dense[*idx2];
                
                (entity, (c1, c2))
            })
        }))
    }
}

impl<'a, C: 'static> Query<'a> for (&'a mut C,) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        world.get_sparse_set_mut::<C>().map(|set| {
            set.iter_mut().map(|(entity, component)| (entity, (component,)))
        })
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a mut C1, &'a mut C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let dense1 = s1.dense.as_mut_ptr();
        let dense2 = s2.dense.as_mut_ptr();

        Some(s1.sparse.iter().filter_map(move |(&entity, &idx1)| {
            let idx2 = s2.get(entity)?;

            Some((entity, unsafe {
                (&mut *dense1.add(idx1), &mut *dense2.add(*idx2))
            }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a C1, &'a mut C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let dense2 = s2.dense.as_mut_ptr();

        Some(s1.iter().filter_map(move |(entity, c1)| {
            let idx2 = s2.get(entity)?;
            
            Some((entity, (c1, unsafe { &mut *dense2.add(*idx2) })))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a mut C1, &'a C2) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;
        let dense1 = s1.dense.as_mut_ptr();

        Some(s2.iter().filter_map(move |(entity, c2)| {
            let idx1 = s1.get(entity)?;
            
            Some((entity, (unsafe { &mut *dense1.add(*idx1) }, c2)))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a C2, &'a C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;
        let s3 = world.get_sparse_set::<C3>()?;

        Some(s1.iter().filter_map(move |(entity, c1)| {
            let c2 = s2.get(entity).and_then(|&idx| s2.dense.get(idx))?;
            let c3 = s3.get(entity).and_then(|&idx| s3.dense.get(idx))?;

            Some((entity, (c1, c2, c3)))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a mut C2, &'a mut C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let s3 = world.get_sparse_set_mut::<C3>()?;
        let dense1 = s1.dense.as_mut_ptr();
        let dense2 = s2.dense.as_mut_ptr();
        let dense3 = s3.dense.as_mut_ptr();

        Some(s1.sparse.iter().filter_map(move |(&entity, &idx1)| {
            let idx2 = s2.get(entity)?;
            let idx3 = s3.get(entity)?;

            Some((entity, unsafe {(
                &mut *dense1.add(idx1),
                &mut *dense2.add(*idx2),
                &mut *dense3.add(*idx3)
            )}))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a C2, &'a C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;
        let s3 = world.get_sparse_set::<C3>()?;
        let dense1 = s1.dense.as_mut_ptr();

        Some(s2.iter().filter_map(move |(entity, c2)| {
            let c3 = s3.get(entity).and_then(|&idx| s3.dense.get(idx))?;
            let idx1 = s1.get(entity)?;

            Some((entity, unsafe { (&mut *dense1.add(*idx1), c2, c3) }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a mut C2, &'a mut C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let s3 = world.get_sparse_set_mut::<C3>()?;
        let dense2 = s2.dense.as_mut_ptr();
        let dense3 = s3.dense.as_mut_ptr();

        Some(s1.iter().filter_map(move |(entity, c1)| {
            let idx2 = s2.get(entity)?;
            let idx3 = s3.get(entity)?;

            Some((entity, unsafe {
                (c1, &mut *dense2.add(*idx2), &mut *dense3.add(*idx3))
            }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a mut C2, &'a C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let s3 = world.get_sparse_set::<C3>()?;
        let dense1 = s1.dense.as_mut_ptr();
        let dense2 = s2.dense.as_mut_ptr();

        Some(s3.iter().filter_map(move |(entity, c3)| {
            let idx1 = s1.get(entity)?;
            let idx2 = s2.get(entity)?;

            Some((entity, unsafe {
                (&mut *dense1.add(*idx1), &mut *dense2.add(*idx2), c3)
            }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a mut C2, &'a C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set_mut::<C2>()?;
        let s3 = world.get_sparse_set::<C3>()?;
        let dense2 = s2.dense.as_mut_ptr();

        Some(s1.iter().filter_map(move |(entity, c1)| {
            let c3 = s3.get(entity).and_then(|&idx| s3.dense.get(idx))?;
            let idx2 = s2.get(entity)?;

            Some((entity, unsafe { (c1, &mut *dense2.add(*idx2), c3) }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a C2, &'a mut C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set_mut::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;
        let s3 = world.get_sparse_set_mut::<C3>()?;
        let dense1 = s1.dense.as_mut_ptr();
        let dense3 = s3.dense.as_mut_ptr();

        Some(s2.iter().filter_map(move |(entity, c2)| {
            let idx1 = s1.get(entity)?;
            let idx3 = s3.get(entity)?;

            Some((entity, unsafe { (&mut *dense1.add(*idx1), c2, &mut *dense3.add(*idx3)) }))
        }))
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a C2, &'a mut C3) {
    fn get_components(world: &'a World) -> Option<impl Iterator<Item = (Entity, Self)> + 'a> {
        let s1 = world.get_sparse_set::<C1>()?;
        let s2 = world.get_sparse_set::<C2>()?;
        let s3 = world.get_sparse_set_mut::<C3>()?;
        let dense3 = s3.dense.as_mut_ptr();

        Some(s1.iter().filter_map(move |(entity, c1)| {
            let c2 = s2.get(entity).and_then(|&idx| s2.dense.get(idx))?;
            let idx3 = s3.get(entity)?;

            Some((entity, unsafe { (c1, c2, &mut *dense3.add(*idx3)) }))
        }))
    }
}