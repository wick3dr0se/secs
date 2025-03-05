use secs::prelude::*;

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn detach_related_in_query() {
    let mut world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|entity, (_,)| world.detach::<u32>(entity));
}

#[test]
fn detach_unrelated_in_query() {
    let mut world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|entity, (_,)| world.detach::<&str>(entity));
}
