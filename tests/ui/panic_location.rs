//@ run: 101

use secs::World;

fn main() {
    let world = World::default();

    let id = world.spawn((1_u32,));
    world.spawn((10_u32, "foo"));
    world.despawn(id);

    world.get::<u32>(id);
}
