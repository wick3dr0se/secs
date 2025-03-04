use secs::prelude::*;

#[test]
#[should_panic(
    expected = "Tried to access component `u32` mutably, but it was already being written to or read from"
)]
fn aliasing_mutation() {
    let mut world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32,));

    // 💥 in miri with stacked borrows
    world.query::<(&mut u32, &mut u32)>(|_, (a, b)| {
        // bad bad bad
        *a = *b;
        *b = *a; // 💥 in miri with tree borrows
    });
}

#[test]
fn optional_components() {
    let mut world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    let mut results = vec![];
    world.query::<(&u32, Option<&&str>)>(|_, (i, s)| results.push((*i, s.map(|s| *s))));
    results.sort();
    assert_eq!(&results[..], &[(1, None), (10, Some("foo"))]);

    let mut results = vec![];
    world.query::<(Option<&&str>, &u32)>(|_, (s, i)| results.push((*i, s.map(|s| *s))));
    results.sort();
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}
