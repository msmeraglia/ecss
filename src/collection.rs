use std::{any::Any, collections::HashMap};

use crate::EntityId;

pub trait Component {
    fn get_type_id() -> u8
    where
        Self: Sized;
    fn get_entity_id(&self) -> EntityId;
}

pub trait EntityCollection: ToAny {
    fn remove(&mut self, entity_id: EntityId);
}

pub trait ToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T: Component + 'static> EntityCollection for Collection<T> {
    fn remove(&mut self, entity_id: EntityId) {
        if self.entity_lookup.contains_key(&entity_id) && !self.data.is_empty() {
            let swap_id = *self.entity_lookup.get(&entity_id).unwrap();
            let last_node_id = self.data.len() - 1;
            if last_node_id > 0 {
                let entity_to_fix = self.entities[last_node_id];
                self.entity_lookup.insert(entity_to_fix, swap_id);
                self.data.swap(swap_id, last_node_id);
                self.entities.swap(swap_id, last_node_id);
            }
            self.entity_lookup.remove(&entity_id);
            self.data.remove(last_node_id);
            self.entities.remove(last_node_id);
        }
    }
}
pub struct Collection<T: Component + 'static> {
    data: Vec<T>,
    entities: Vec<EntityId>,
    entity_lookup: HashMap<EntityId, usize>,
}

impl<T: 'static + Component> Default for Collection<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            entities: Vec::new(),
            entity_lookup: HashMap::new(),
        }
    }
}

impl<T: 'static + Component> Collection<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            entities: Vec::with_capacity(size),
            entity_lookup: HashMap::with_capacity(size),
        }
    }

    pub fn contains(&self, entity_id: EntityId) -> bool {
        self.entity_lookup.contains_key(&entity_id)
    }

    pub fn create(&mut self, data: T) {
        let entity_id = data.get_entity_id();
        if !self.entity_lookup.contains_key(&entity_id) && self.data.len() < self.data.capacity() {
            self.entity_lookup.insert(entity_id, self.data.len());
            self.entities.push(entity_id);
            self.data.push(data);
        }
    }

    pub fn entities_where<F>(&self, f: F) -> Vec<EntityId>
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.data
        .iter()
        .filter(|&data| f(data))
        .map(|data|data.get_entity_id())
        .collect()
    }

    pub fn get(&self, entity_id: EntityId) -> Option<&T> {
        if let Some(data_id) = self.entity_lookup.get(&entity_id) {
            return self.data.get(*data_id);
        }
        None
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<&mut T> {
        if let Some(data_id) = self.entity_lookup.get(&entity_id) {
            return self.data.get_mut(*data_id);
        }
        None
    }

    pub fn get_entities(&self) -> Vec<EntityId> {
        self.entities.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= self.data.capacity()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_with_entities(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.entities
            .iter()
            .zip(self.data.iter())
            .map(|(entity, item)| (*entity, item))
            .into_iter()
    }

    pub fn iter_with_entities_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.entities
            .iter()
            .zip(self.data.iter_mut())
            .map(|(entity, item)| (*entity, item))
            .into_iter()
    }
}