use secs::World;

fn optional_components() {
    let world = World::default();

    let mut results = vec![];
    world.query(|_, _: &u32, _: &str| {});
    //~^ ERROR: is not a valid query

    world.query(|_, _: u32, _: &&str| {});
    //~^ ERROR: is not a valid query
}
