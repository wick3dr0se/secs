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
