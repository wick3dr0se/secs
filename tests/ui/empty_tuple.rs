use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    world.query::<()>(|_, ()| {});
    //~^ ERROR: `()` is not a valid query
}
