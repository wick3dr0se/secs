use secs::World;

fn optional_components() {
    let world = World::default();

    world.query(
        |_,
         //~^ ERROR: is not a valid query
         _: &u32,
         _: &i32,
         _: &u8,
         _: &i8,
         _: &u64,
         _: &i64,
         _: &u16,
         _: &i16,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &u128,
         _: &i128| {},
    );
}
