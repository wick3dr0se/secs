use secs::World;

fn optional_components() {
    let world = World::default();

    world.query::<(u32, i32, u8, i8, u64, i64, u16, i16, u128, i128)>(|_, ()| {});
    //~^ ERROR: `(u32, i32, u8, i8, u64, i64, u16, i16, u128, i128)` is not a valid query
}
