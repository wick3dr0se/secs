use secs::prelude::*;

fn optional_components() {
    let world = World::default();

    world.query::<()>(|_, ()| {});
    //~^ ERROR: `()` is not a valid query
}
