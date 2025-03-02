use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

use crate::{
    sparse_set::SparseSet,
    world::{Entity, World, WorldQuery},
};

pub trait Query<'a>: Sized {
    type Short<'b, 'c, 'd, 'e, 'f>;

    #[track_caller]
    fn get_components(
        world: &'a World,
        f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    );
}

/// A helper that allows more copy paste
pub trait SparseSetGetter<'a> {
    type Short<'b>;
    type Iter;
    fn get_set(world: &'a World) -> Option<Self::Iter>;
    fn get_entity(iter: &mut Self::Iter, entity: Entity) -> Option<Self::Short<'_>>;
    fn iter(iter: &mut Self::Iter) -> impl Iterator<Item = (Entity, Self::Short<'_>)>;
}

impl<'a, C: 'static> SparseSetGetter<'a> for &'a C {
    type Short<'b> = &'b C;
    type Iter = MappedRwLockReadGuard<'a, SparseSet<C>>;
    fn get_set(world: &'a World) -> Option<Self::Iter> {
        world.get_sparse_set()
    }
    fn get_entity(iter: &mut Self::Iter, entity: Entity) -> Option<Self::Short<'_>> {
        let id = iter.get(entity)?;
        Some(&iter.dense[id])
    }
    fn iter(iter: &mut Self::Iter) -> impl Iterator<Item = (Entity, Self::Short<'_>)> {
        (&**iter).into_iter()
    }
}

impl<'a, C: 'static> SparseSetGetter<'a> for &'a mut C {
    type Short<'b> = &'b mut C;
    type Iter = MappedRwLockWriteGuard<'a, SparseSet<C>>;
    fn get_set(world: &'a World) -> Option<Self::Iter> {
        world.get_sparse_set_mut()
    }
    fn get_entity(iter: &mut Self::Iter, entity: Entity) -> Option<Self::Short<'_>> {
        let id = iter.get(entity)?;
        Some(&mut iter.dense[id])
    }
    fn iter(iter: &mut Self::Iter) -> impl Iterator<Item = (Entity, Self::Short<'_>)> {
        (&mut **iter).into_iter()
    }
}

impl<'a, T: SparseSetGetter<'a> + 'a> Query<'a> for (T,) {
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>,);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        let Some(mut s1) = T::get_set(world) else {
            return;
        };

        for (entity, c1) in T::iter(&mut s1) {
            f(entity, (c1,));
        }
    }
}

impl<'a, T: SparseSetGetter<'a> + 'a, U: SparseSetGetter<'a> + 'a> Query<'a> for (T, U) {
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>, U::Short<'c>);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        let Some(mut s1) = T::get_set(world) else {
            return;
        };
        let Some(mut s2) = U::get_set(world) else {
            return;
        };

        for (entity, c1) in T::iter(&mut s1) {
            let Some(c2) = U::get_entity(&mut s2, entity) else {
                continue;
            };
            f(entity, (c1, c2));
        }
    }
}

impl<'a, T: SparseSetGetter<'a> + 'a, U: SparseSetGetter<'a> + 'a, V: SparseSetGetter<'a> + 'a>
    Query<'a> for (T, U, V)
{
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>, U::Short<'c>, V::Short<'d>);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        let Some(mut s1) = T::get_set(world) else {
            return;
        };
        let Some(mut s2) = U::get_set(world) else {
            return;
        };
        let Some(mut s3) = V::get_set(world) else {
            return;
        };

        for (entity, c1) in T::iter(&mut s1) {
            let Some(c2) = U::get_entity(&mut s2, entity) else {
                continue;
            };
            let Some(c3) = V::get_entity(&mut s3, entity) else {
                continue;
            };
            f(entity, (c1, c2, c3));
        }
    }
}

impl<
    'a,
    T: SparseSetGetter<'a> + 'a,
    U: SparseSetGetter<'a> + 'a,
    V: SparseSetGetter<'a> + 'a,
    W: SparseSetGetter<'a> + 'a,
> Query<'a> for (T, U, V, W)
{
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>, U::Short<'c>, V::Short<'d>, W::Short<'e>);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        let Some(mut s1) = T::get_set(world) else {
            return;
        };
        let Some(mut s2) = U::get_set(world) else {
            return;
        };
        let Some(mut s3) = V::get_set(world) else {
            return;
        };
        let Some(mut s4) = W::get_set(world) else {
            return;
        };

        for (entity, c1) in T::iter(&mut s1) {
            let Some(c2) = U::get_entity(&mut s2, entity) else {
                continue;
            };
            let Some(c3) = V::get_entity(&mut s3, entity) else {
                continue;
            };
            let Some(c4) = W::get_entity(&mut s4, entity) else {
                continue;
            };
            f(entity, (c1, c2, c3, c4));
        }
    }
}

impl<
    'a,
    T: SparseSetGetter<'a> + 'a,
    U: SparseSetGetter<'a> + 'a,
    V: SparseSetGetter<'a> + 'a,
    W: SparseSetGetter<'a> + 'a,
    X: SparseSetGetter<'a> + 'a,
> Query<'a> for (T, U, V, W, X)
{
    type Short<'b, 'c, 'd, 'e, 'f> = (
        T::Short<'b>,
        U::Short<'c>,
        V::Short<'d>,
        W::Short<'e>,
        X::Short<'f>,
    );

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        let Some(mut s1) = T::get_set(world) else {
            return;
        };
        let Some(mut s2) = U::get_set(world) else {
            return;
        };
        let Some(mut s3) = V::get_set(world) else {
            return;
        };
        let Some(mut s4) = W::get_set(world) else {
            return;
        };
        let Some(mut s5) = X::get_set(world) else {
            return;
        };

        for (entity, c1) in T::iter(&mut s1) {
            let Some(c2) = U::get_entity(&mut s2, entity) else {
                continue;
            };
            let Some(c3) = V::get_entity(&mut s3, entity) else {
                continue;
            };
            let Some(c4) = W::get_entity(&mut s4, entity) else {
                continue;
            };
            let Some(c5) = X::get_entity(&mut s5, entity) else {
                continue;
            };
            f(entity, (c1, c2, c3, c4, c5));
        }
    }
}
