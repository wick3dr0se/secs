use std::panic::AssertUnwindSafe;

use secs::World;

#[test]
fn remove() {
    let mut world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    // Miri will assume every time you mention a function you could have a new address
    // While this is bogus in many cases, it's the only way to catch the rare cases where
    // it's an issue. So we do this provenance preserving deduplication by eagerly creating a pointer.
    let boom = boom as fn(&World);
    world.add_system(boom);
    assert!(std::panic::catch_unwind(AssertUnwindSafe(|| world.run_systems())).is_err());
    world.remove_system(boom);
    world.run_systems();
}

#[test]
fn remove_within() {
    let mut world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    // Miri will assume every time you mention a function you could have a new address
    // While this is bogus in many cases, it's the only way to catch the rare cases where
    // it's an issue. So we do this provenance preserving deduplication by eagerly creating a pointer.
    let boom = boom as fn(&World);
    world.add_resource(boom);

    fn defuse(world: &mut World) {
        let boom = *world.get_resource::<fn(&World)>().unwrap();
        // Remove the boom system. Since we run before it, it will never actuall run, yay.
        world.remove_system(boom);
    }

    world.add_mut_system(defuse);
    world.add_system(boom);
    world.run_systems();
}

#[test]
fn despawn() {
    let mut world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    let mut results = vec![];
    world.query::<(&u32, Option<&&str>)>(|_, (i, s)| results.push((*i, s.map(|s| *s))));
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}

#[test]
fn get() {
    let mut world = World::default();

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
    let mut world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    let mut results = vec![];
    world.query::<(&mut u32,)>(|entity, (i,)| results.push((*i, world.get(entity).map(|s| *s))));
    assert_eq!(&results[..], &[(10, Some(0_u32))]);
}
