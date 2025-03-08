use secs::World;

fn optional_components() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    let mut results = vec![];
    world.query::<(&u32, Option<&&str>)>(|_, (i, s)| results.push((*i, s.map(|s| *s))));
    results.sort();
    assert_eq!(&results[..], &[(1, None), (10, Some("foo"))]);

    let mut results = vec![];
    world.query::<(Option<&&str>, &u32)>(|_, (s, i)| results.push((*i, s.map(|s| *s))));
    //~^ ERROR: `Option<&&str>` cannot be the first element of a query
    results.sort();
    assert_eq!(&results[..], &[(10, Some("foo"))]);
}
