use secs::prelude::*;

fn optional_components() {
    let world = World::default();

    world.query::<u32>(|_, _| {});
    //~^ ERROR: `u32` is not a valid query
}
