mod query;
mod scheduler;
mod sparse_set;
mod world;
mod components;

pub mod prelude {
    pub use super::world::{Entity, World};
    pub use super::scheduler::ExecutionMode;
}