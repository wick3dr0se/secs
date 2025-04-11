use secs::World;

fn optional_components() {
    let world = World::default();

    world.query(42_u32);
    //~^ ERROR: `u32` is not a valid query
}
