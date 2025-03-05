use secs::prelude::*;

fn optional_components() {
    let mut world = World::default();

    world.query::<(u32, i32, u8, i8, u64, i64, u16, i16, u128, i128)>(|_, ()| {});
    //~^ ERROR: the trait bound `(u32, i32, u8, i8, u64, i64, u16, i16, u128, i128): secs::query::Query<'_>` is not satisfied
}
