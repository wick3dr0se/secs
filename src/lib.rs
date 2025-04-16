//! A very minimalistic entity component system.
//!
//! Start by creating a [World] and invoke methods on it
//! to fill your world with life.

mod query;
mod scheduler;
mod sparse_set;
mod world;

pub use scheduler::Scheduler;
pub use scheduler::SysId;
pub use world::{Entity, World};
