mod collection;
pub mod ecss;

// Public
pub type Entity = usize; //TODO Make struct including generational id
pub static INVALID_ENTITY: Entity = 0;

pub use crate::ecss::ECSS;

// Private
use collection::Collection;

pub use ecss_component_derive::Component;

pub trait Component {
    fn get_entity(&self) -> Entity;
}
