use crate::{Component, Entity};
use std::collections::HashMap;

//TODO: Eventually allow client to pass custom allocator for data Vec
pub struct Collection<T: 'static + Component + Sized> {
    entity_lookup: HashMap<Entity, usize>,
    data: Vec<T>,
    entities: Vec<Entity>,
}

impl<T: 'static + Component + Sized> Default for Collection<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            entities: Vec::new(),
            entity_lookup: HashMap::new(),
        }
    }
}

impl<T: 'static + Component + Sized> Collection<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            entities: Vec::with_capacity(size),
            entity_lookup: HashMap::with_capacity(size),
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entity_lookup.contains_key(&entity)
    }

    pub fn create(&mut self, entity: Entity, data: T) {
        if !self.entity_lookup.contains_key(&entity) && self.data.len() < self.data.capacity() {
            self.entity_lookup.insert(entity, self.data.len());
            self.data.push(data);
            self.entities.push(entity);
        }
    }

    pub fn entities_where<F>(&self, f: F) -> Vec<Entity>
    where
        F: Fn(&T) -> bool + 'static,
    {
        let mut entities = Vec::new();
        for (i, item) in self.data.iter().enumerate() {
            if f(item) {
                entities.push(self.entities[i]);
            }
        }
        entities
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if let Some(data_id) = self.entity_lookup.get(&entity) {
            return self.data.get(*data_id);
        }
        None
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if let Some(data_id) = self.entity_lookup.get(&entity) {
            return self.data.get_mut(*data_id);
        }
        None
    }

    pub fn get_entities(&self) -> Vec<Entity> {
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

    pub fn iter_with_entities(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities
            .iter()
            .zip(self.data.iter())
            .map(|(entity, item)| (*entity, item))
            .into_iter()
    }

    pub fn iter_with_entities_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.entities
            .iter()
            .zip(self.data.iter_mut())
            .map(|(entity, item)| (*entity, item))
            .into_iter()
    }

    pub fn remove(&mut self, entity: Entity) {
        if self.entity_lookup.contains_key(&entity) && !self.data.is_empty() {
            let swap_id = *self.entity_lookup.get(&entity).unwrap();
            let last_node_id = self.data.len() - 1;
            if last_node_id > 0 {
                let entity_to_fix = self.entities[last_node_id];
                self.entity_lookup.insert(entity_to_fix, swap_id);
                self.data.swap(swap_id, last_node_id);
                self.entities.swap(swap_id, last_node_id);
            }
            self.entity_lookup.remove(&entity);
            self.data.remove(last_node_id);
            self.entities.remove(last_node_id);
        }
    }
}
