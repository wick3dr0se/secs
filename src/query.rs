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

macro_rules! impl_query {
    ($($T:ident),*) => {
        impl<A: SparseSetGetter + Always, $($T: SparseSetGetter,)* Z> Query<(A, $($T,)*)> for Z
where
    Z: FnMut(Entity, A::Short<'_>, $($T::Short<'_>,)*) + FnMut(Entity, A, $($T,)*),{
            #[track_caller]
            fn get_components(world: &World, mut f: Z) {
                #[allow(non_snake_case)]
                if let (Some(mut a), $(Some(mut $T),)*) = (A::get_set(world), $($T::get_set(world),)*) {
                    for (entity, a) in A::iter(&mut a) {
                        $(let Some($T) = $T::get_entity(&mut $T, entity) else { continue };)*
                        f(entity, a, $($T,)*);

                    }
                }
            }
        }
    };
}

impl_query!();
impl_query!(B);
impl_query!(B, C);
impl_query!(B, C, D);
impl_query!(B, C, D, E);
impl_query!(B, C, D, E, F);
impl_query!(B, C, D, E, F, G);
impl_query!(B, C, D, E, F, G, H);
impl_query!(B, C, D, E, F, G, H, I);
impl_query!(B, C, D, E, F, G, H, I, J);
impl_query!(B, C, D, E, F, G, H, I, J, K);
impl_query!(B, C, D, E, F, G, H, I, J, K, L);
