pub mod ecss;
pub use crate::ecss::{Entity, ECSS};

mod collection;
use collection::{Collection, Component, ComponentIdType, EntityCollection};
