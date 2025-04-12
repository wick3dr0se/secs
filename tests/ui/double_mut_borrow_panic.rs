//@run:101

use secs::World;

fn main() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32,));

    world.query(|_, _: &mut u32, _: &mut u32| {});
}
