use secs::World;

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn aliasing_mutation() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32,));

    // 💥 in miri with stacked borrows
    world.query(|_, a: &mut u32, b: &mut u32| {
        // bad bad bad
        *a = *b;
        *b = *a; // 💥 in miri with tree borrows
    });
}

#[test]
fn optional_components() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    let mut results = vec![];
    world.query(|_, i: &u32, s: Option<&&'static str>| results.push((*i, s.map(|s| *s))));
    results.sort();
    assert_eq!(&results[..], &[(1, None), (10, Some("foo"))]);

    let mut results = vec![];
    world.query(|_, s: &&'static str, i: &u32| results.push((*i, *s)));
    results.sort();
    assert_eq!(&results[..], &[(10, "foo")]);
}
