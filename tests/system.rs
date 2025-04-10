use std::{
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
};

use secs::{SysId, World};

#[test]
fn remove() {
    let world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    let id: SysId = world.add_system(boom);
    assert!(std::panic::catch_unwind(AssertUnwindSafe(|| world.run_systems())).is_err());
    world.remove_system(id);
    world.run_systems();
}

#[test]
fn remove_within() {
    let world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    let id = Arc::new(Mutex::new(None));
    let id2 = id.clone();
    world.add_system(move |world| {
        world.remove_system(id.lock().unwrap().unwrap());
    });

    *id2.lock().unwrap() = Some(world.add_system(boom));
    world.run_systems();
}

#[test]
fn despawn() {
    let world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    let mut results = vec![];
    world.query::<(&u32, Option<&&str>)>(|_, (i, s)| results.push((*i, s.map(|s| *s))));
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}

#[test]
fn get() {
    let world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    let mut results = vec![];
    world.query::<(&u32,)>(|entity, (i,)| results.push((*i, world.get(entity).map(|s| *s))));
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}

#[test]
#[should_panic(expected = "Tried to access component `u32`, but it was already being written to")]
fn get_fail() {
    let world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    let mut results = vec![];
    world.query::<(&mut u32,)>(|entity, (i,)| results.push((*i, world.get(entity).map(|s| *s))));
    assert_eq!(&results[..], &[(10, Some(0_u32))]);
}

#[test]
fn query_system() {
    let world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));

    world.add_query_system::<(&mut u32,)>(|_world, _entity, (i,)| {
        *i *= 2;
    });

    let mut state = 5_u32;

    world.add_system(move |_world| {
        state += 1;
        assert!(state <= 8);
    });

    for _ in 0..3 {
        world.run_systems();
    }
    let i = world.get::<u32>(id).unwrap();
    assert_eq!(*i, 8);
}
