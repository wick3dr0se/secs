use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    let mut results = vec![];
    world.query::<(&u32, &str)>(|_, _| {});
    //~^ ERROR: the size for values of type `str` cannot be known at compilation time

    world.query::<(u32, &&str)>(|_, _| {});
    //~^ ERROR: `u32` cannot be used as a query component
    //~| ERROR: `u32` cannot be used as a query component
    //~| ERROR: `u32` cannot be the first element of a query
}
