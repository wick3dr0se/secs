use secs::prelude::*;

#[test]
fn aliasing_mutation() {
    let mut world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32,));

    // 💥 in miri with stacked borrows
    for (_, (a, b)) in world.query::<(&mut u32, &mut u32)>() {
        // bad bad bad
        *a = *b;
        *b = *a; // 💥 in miri with tree borrows
    }
}
