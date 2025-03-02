use std::panic::AssertUnwindSafe;

use secs::prelude::*;

#[test]
fn remove() {
    let mut world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    world.add_system(boom, ExecutionMode::Serial);
    assert!(std::panic::catch_unwind(AssertUnwindSafe(|| world.run_systems())).is_err());
    world.remove_system(boom);
    world.run_systems();
}
