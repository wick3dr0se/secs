use secs::prelude::*;

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn detach_related_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|entity, (_,)| {
        world.detach::<u32>(entity);
    });
}

#[test]
fn detach_unrelated_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|entity, (_,)| {
        world.detach::<&str>(entity);
    });
}

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn spawn_related_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|_, (&i,)| {
        world.spawn((i * 2,));
    });
}

#[test]
fn spawn_unrelated_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query::<(&u32,)>(|_, (_,)| {
        world.spawn(("bar",));
    });
}

#[test]
fn detach() {
    let world = World::default();
    let entity = world.spawn((String::new(),));
    world.detach::<String>(entity).unwrap();
    assert_eq!(None, world.detach::<String>(entity));
}
