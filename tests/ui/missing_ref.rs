use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    let mut results = vec![];
    world.query::<(&u32, &str)>(|_, _| {});
    //~^ ERROR: the size for values of type `str` cannot be known at compilation time

    world.query::<(u32, &&str)>(|_, _| {});
    //~^ ERROR: the trait bound `u32: secs::query::SparseSetGetter<'_>` is not satisfied
    //~| ERROR: the trait bound `u32: secs::query::SparseSetGetter<'_>` is not satisfied
    //~| ERROR: `u32` cannot be the first element of a query
}
