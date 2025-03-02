use std::panic::AssertUnwindSafe;

use secs::prelude::*;

#[test]
fn remove() {
    let mut world = World::default();
    fn boom(_: &World) {
        panic!()
    }
    // Miri will assume every time you mention a function you could have a new address
    // While this is bogus in many cases, it's the only way to catch the rare cases where
    // it's an issue. So we do this provenance preserving deduplication by eagerly creating a pointer.
    let boom = boom as fn(&World);
    world.add_system(boom, ExecutionMode::Serial);
    assert!(std::panic::catch_unwind(AssertUnwindSafe(|| world.run_systems())).is_err());
    world.remove_system(boom);
    world.run_systems();
}
