use secs::World;

#[test]
fn check_component_attached() {
    let world = World::default();

    let entity = world.spawn((1_u32,));

    assert!(world.is_attached::<u32>(entity));

    world.detach::<u32>(entity);
    assert!(!world.is_attached::<u32>(entity));
}

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn detach_related_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query(|entity, _: &u32| {
        world.detach::<u32>(entity);
    });
}

#[test]
fn detach_unrelated_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query(|entity, _: &u32| {
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
    world.query(|_, &i: &u32| {
        world.spawn((i * 2,));
    });
}

#[test]
#[should_panic(expected = "Attaching `alloc::string::String` to despawned entity")]
#[cfg(any(debug_assertions, feature = "track_dead_entities"))]
fn attach_to_despawned() {
    let world = World::default();

    let entity = world.spawn((1_u32,));
    world.despawn(entity);
    world.flush_despawned();
    world.attach(entity, (String::new(),));
}

#[test]
fn attach_to_despawned_no_flush() {
    let world = World::default();

    let entity = world.spawn((1_u32,));
    world.despawn(entity);
    world.attach(entity, (String::new(),));
}

#[test]
fn spawn_unrelated_in_query() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.query(|_, _: &u32| {
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

#[test]
fn detach_any() {
    let world = World::default();
    let entity = world.spawn((1_u32, "foo"));

    world.detach_any::<u32>();

    assert!(!world.is_attached::<u32>(entity));
}
