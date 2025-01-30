mod query;
mod scheduler;
mod sparse_set;
mod world;

pub mod prelude {
    pub use super::world::{Entity, World};
    pub use super::scheduler::ExecutionMode;
}