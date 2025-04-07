//! A very minimalistic entity component system.
//!
//! Start by creating a [World] and invoke methods on it
//! to fill your world with life.

#[cfg(not(feature = "multithreaded"))]
use elsa::FrozenVec;
#[cfg(feature = "multithreaded")]
use elsa::sync::FrozenVec;

mod components;
mod query;
mod scheduler;
mod sparse_set;
mod world;

pub use scheduler::SysId;
pub use world::{Entity, World};
