//@run:101

use secs::World;

fn main() {
    let world = World::default();

    world.spawn((1_u32,));
    world.spawn((10_u32,));

    world.query::<(&mut u32, &mut u32)>(|_, (_, _)| {});
}
