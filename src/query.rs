use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

use crate::{
    sparse_set::SparseSet,
    world::{Entity, World},
};

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid query",
    label = "",
    note = "only tuples with 1 or up to 5 elements can be used as queries"
)]
pub trait Query<'a>: Sized {
    type Short<'b, 'c, 'd, 'e, 'f>;

    #[track_caller]
    fn get_components(
        world: &'a World,
        f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    );
}

/// A marker trait preventing `Option` from being used as the first field in a query tuple.
/// This needs to be prevented, as it does not iterate over all entity ids, but only the ones
/// within that list, always producing `Some`. In that case you can just leave off the `Option`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be the first element of a query",
    label = "",
    note = "move another element to the front of the list"
)]
pub trait Always {}

impl<T> Always for &T {}
impl<T> Always for &mut T {}

/// A helper that allows more copy paste
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as a query component",
    label = "",
    note = "only references and `Option`s of references can be components"
)]
pub trait SparseSetGetter<'a> {
    type Short<'b>;
    type Iter;
    fn get_set(world: &'a World) -> Option<Self::Iter>;
    fn get_entity(iter: &mut Self::Iter, entity: Entity) -> Option<Self::Short<'_>>;
    fn iter(iter: &mut Self::Iter) -> impl Iterator<Item = (Entity, Self::Short<'_>)>
    where
        Self: Always;
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

impl<'a, T: SparseSetGetter<'a>> SparseSetGetter<'a> for Option<T> {
    type Short<'b> = Option<T::Short<'b>>;
    type Iter = T::Iter;
    fn get_set(world: &'a World) -> Option<Self::Iter> {
        T::get_set(world)
    }
    fn get_entity(iter: &mut Self::Iter, entity: Entity) -> Option<Self::Short<'_>> {
        Some(T::get_entity(iter, entity))
    }
    fn iter(_iter: &mut Self::Iter) -> impl Iterator<Item = (Entity, Self::Short<'_>)>
    where
        Self: Always,
    {
        std::iter::once_with(|| unreachable!())
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

impl<'a, T: SparseSetGetter<'a> + Always + 'a> Query<'a> for (T,) {
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>,);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        if let Some(mut s1) = T::get_set(world) {
            for (entity, c1) in T::iter(&mut s1) {
                f(entity, (c1,));
            }
        }
    }
}

impl<'a, T: SparseSetGetter<'a> + Always + 'a, U: SparseSetGetter<'a> + 'a> Query<'a> for (T, U) {
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>, U::Short<'c>);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        if let (Some(mut s1), Some(mut s2)) = (T::get_set(world), U::get_set(world)) {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    f(entity, (c1, c2));
                }
            }
        }
    }
}

impl<
    'a,
    T: SparseSetGetter<'a> + Always + 'a,
    U: SparseSetGetter<'a> + 'a,
    V: SparseSetGetter<'a> + 'a,
> Query<'a> for (T, U, V)
{
    type Short<'b, 'c, 'd, 'e, 'f> = (T::Short<'b>, U::Short<'c>, V::Short<'d>);

    #[track_caller]
    fn get_components(
        world: &'a World,
        mut f: impl for<'b, 'c, 'd, 'e, 'f> FnMut(Entity, Self::Short<'b, 'c, 'd, 'e, 'f>),
    ) {
        if let (Some(mut s1), Some(mut s2), Some(mut s3)) =
            (T::get_set(world), U::get_set(world), V::get_set(world))
        {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    if let Some(c3) = V::get_entity(&mut s3, entity) {
                        f(entity, (c1, c2, c3));
                    }
                }
            }
        }
    }
}

impl<
    'a,
    T: SparseSetGetter<'a> + Always + 'a,
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
        if let (Some(mut s1), Some(mut s2), Some(mut s3), Some(mut s4)) = (
            T::get_set(world),
            U::get_set(world),
            V::get_set(world),
            W::get_set(world),
        ) {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    if let Some(c3) = V::get_entity(&mut s3, entity) {
                        if let Some(c4) = W::get_entity(&mut s4, entity) {
                            f(entity, (c1, c2, c3, c4));
                        }
                    }
                }
            }
        }
    }
}

impl<
    'a,
    T: SparseSetGetter<'a> + Always + 'a,
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
        if let (Some(mut s1), Some(mut s2), Some(mut s3), Some(mut s4), Some(mut s5)) = (
            T::get_set(world),
            U::get_set(world),
            V::get_set(world),
            W::get_set(world),
            X::get_set(world),
        ) {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    if let Some(c3) = V::get_entity(&mut s3, entity) {
                        if let Some(c4) = W::get_entity(&mut s4, entity) {
                            if let Some(c5) = X::get_entity(&mut s5, entity) {
                                f(entity, (c1, c2, c3, c4, c5));
                            }
                        }
                    }
                }
            }
        }
    }
}
