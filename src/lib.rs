#[macro_use]
pub mod ecss;
pub use crate::ecss::{EntityId, ECSS};

mod collection;
use collection::{Collection, Component, EntityCollection};
