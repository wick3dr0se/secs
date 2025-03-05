use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    world.query::<u32>(|_, _| {});
    //~^ ERROR: the trait bound `u32: secs::query::Query<'_>` is not satisfied
}
