use std::cell::{Ref, RefMut};

use crate::{
    sparse_set::SparseSet,
    world::{Entity, World},
};

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid query",
    label = "",
    note = "only tuples with 1 or up to 5 elements can be used as queries"
)]
pub trait Query<ARGS>: Sized {
    #[track_caller]
    fn get_components(world: &World, f: Self);
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
pub trait SparseSetGetter {
    type Short<'b>;
    type Iter<'c>;
    fn get_set(world: &World) -> Option<Self::Iter<'_>>;
    fn get_entity<'b>(iter: &'b mut Self::Iter<'_>, entity: Entity) -> Option<Self::Short<'b>>;
    fn iter<'b>(iter: &'b mut Self::Iter<'_>) -> impl Iterator<Item = (Entity, Self::Short<'b>)>
    where
        Self: Always;
}

impl<C: 'static> SparseSetGetter for &C {
    type Short<'b> = &'b C;
    type Iter<'c> = Ref<'c, SparseSet<C>>;
    #[track_caller]
    fn get_set(world: &World) -> Option<Self::Iter<'_>> {
        world.sparse_sets.get()
    }
    fn get_entity<'b>(iter: &'b mut Self::Iter<'_>, entity: Entity) -> Option<Self::Short<'b>> {
        iter.get(entity)
    }
    fn iter<'b>(iter: &'b mut Self::Iter<'_>) -> impl Iterator<Item = (Entity, Self::Short<'b>)> {
        iter.iter()
    }
}

impl<T: SparseSetGetter> SparseSetGetter for Option<T> {
    type Short<'b> = Option<T::Short<'b>>;
    type Iter<'c> = T::Iter<'c>;
    #[track_caller]
    fn get_set(world: &World) -> Option<Self::Iter<'_>> {
        T::get_set(world)
    }
    fn get_entity<'b>(iter: &'b mut Self::Iter<'_>, entity: Entity) -> Option<Self::Short<'b>> {
        Some(T::get_entity(iter, entity))
    }
    fn iter<'b>(_iter: &'b mut Self::Iter<'_>) -> impl Iterator<Item = (Entity, Self::Short<'b>)>
    where
        Self: Always,
    {
        std::iter::once_with(|| unreachable!())
    }
}

impl<C: 'static> SparseSetGetter for &mut C {
    type Short<'b> = &'b mut C;
    type Iter<'c> = RefMut<'c, SparseSet<C>>;
    #[track_caller]
    fn get_set(world: &World) -> Option<Self::Iter<'_>> {
        world.sparse_sets.get_mut()
    }
    fn get_entity<'b>(iter: &'b mut Self::Iter<'_>, entity: Entity) -> Option<Self::Short<'b>> {
        iter.get_mut(entity)
    }
    fn iter<'b>(iter: &'b mut Self::Iter<'_>) -> impl Iterator<Item = (Entity, Self::Short<'b>)> {
        iter.iter_mut()
    }
}

impl<T: SparseSetGetter + Always, F> Query<(T,)> for F
where
    F: FnMut(Entity, T::Short<'_>) + FnMut(Entity, T),
{
    #[track_caller]
    fn get_components(world: &World, mut f: F) {
        if let Some(mut s1) = T::get_set(world) {
            for (entity, c1) in T::iter(&mut s1) {
                f(entity, c1);
            }
        }
    }
}

impl<T: SparseSetGetter + Always, U: SparseSetGetter, F> Query<(T, U)> for F
where
    F: FnMut(Entity, T::Short<'_>, U::Short<'_>) + FnMut(Entity, T, U),
{
    #[track_caller]
    fn get_components(world: &World, mut f: F) {
        if let (Some(mut s1), Some(mut s2)) = (T::get_set(world), U::get_set(world)) {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    f(entity, c1, c2);
                }
            }
        }
    }
}

impl<T: SparseSetGetter + Always, U: SparseSetGetter, V: SparseSetGetter, F> Query<(T, U, V)> for F
where
    F: FnMut(Entity, T::Short<'_>, U::Short<'_>, V::Short<'_>) + FnMut(Entity, T, U, V),
{
    #[track_caller]
    fn get_components(world: &World, mut f: F) {
        if let (Some(mut s1), Some(mut s2), Some(mut s3)) =
            (T::get_set(world), U::get_set(world), V::get_set(world))
        {
            for (entity, c1) in T::iter(&mut s1) {
                if let Some(c2) = U::get_entity(&mut s2, entity) {
                    if let Some(c3) = V::get_entity(&mut s3, entity) {
                        f(entity, c1, c2, c3);
                    }
                }
            }
        }
    }
}

impl<T: SparseSetGetter + Always, U: SparseSetGetter, V: SparseSetGetter, W: SparseSetGetter, F>
    Query<(T, U, V, W)> for F
where
    F: FnMut(Entity, T::Short<'_>, U::Short<'_>, V::Short<'_>, W::Short<'_>)
        + FnMut(Entity, T, U, V, W),
{
    #[track_caller]
    fn get_components(world: &World, mut f: F) {
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
                            f(entity, c1, c2, c3, c4);
                        }
                    }
                }
            }
        }
    }
}

impl<
    T: SparseSetGetter + Always,
    U: SparseSetGetter,
    V: SparseSetGetter,
    W: SparseSetGetter,
    X: SparseSetGetter,
    F,
> Query<(T, U, V, W, X)> for F
where
    F: FnMut(Entity, T::Short<'_>, U::Short<'_>, V::Short<'_>, W::Short<'_>, X::Short<'_>)
        + FnMut(Entity, T, U, V, W, X),
{
    #[track_caller]
    fn get_components(world: &World, mut f: F) {
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
                                f(entity, c1, c2, c3, c4, c5);
                            }
                        }
                    }
                }
            }
        }
    }
}
