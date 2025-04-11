use secs::World;

fn optional_components() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    let mut results = vec![];
    world.query(|_, i: &u32, s: Option<&&str>| results.push((*i, s.map(|s| *s))));
    results.sort();
    assert_eq!(&results[..], &[(1, None), (10, Some("foo"))]);

    let mut results = vec![];
    world.query(|_, s: Option<&&str>, i: &u32| results.push((*i, s.map(|s| *s))));
    //~^ ERROR: is not a valid query
    results.sort();
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}
