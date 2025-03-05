use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    world.query::<()>(|_, ()| {});
    //~^ ERROR: the trait bound `(): secs::query::Query<'_>` is not satisfied
}
