pub mod ecss;
pub use crate::ecss::{Entity, ECSS, Type, ComponentType};

mod collection;
use collection::Collection;

pub use ecss_component_derive::Component;
pub trait Component {
    fn entity_id(&self) -> Entity;
}