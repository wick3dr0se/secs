//! A very minimalistic entity component system.
//!
//! Start by creating a [World](world::World) and invoke methods on it
//! to fill your world with life.

mod components;
mod query;
mod scheduler;
mod sparse_set;
mod world;

/// The one and only way to import things from this crate.
pub mod prelude {
    pub use super::world::{Entity, World};
}
