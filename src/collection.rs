use crate::{Component, Entity};

use std::collections::HashMap;

//TODO: Eventually allow client to pass custom allocator for data Vec
pub struct Collection<T: Component> {
    entity_lookup: HashMap<Entity, usize>,
    data: Vec<T>,
}

impl<T: Component> Default for Collection<T> {
    fn default() -> Self {
        Self {
            entity_lookup: HashMap::new(),
            data: Vec::new(),
        }
    }
}

impl<T: Component> Collection<T> {
    pub fn new(size: usize) -> Self {
        Self {
            entity_lookup: HashMap::with_capacity(size),
            data: Vec::with_capacity(size),
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entity_lookup.contains_key(&entity)
    }

    //TODO: What happens when its full? Return a Result<>, Option<>? Panic?
    //This should definitely indicate to the use its full, perhaps add a flag for if Vecs can resize..
    pub fn create(&mut self, data: T) {
        let entity = data.entity_id();
        if !self.entity_lookup.contains_key(&entity) && self.data.len() < self.data.capacity() {
            self.entity_lookup.insert(entity, self.data.len());
            self.data.push(data);
        }
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
        self.entity_lookup.keys().copied().collect()
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

    pub fn remove(&mut self, entity: Entity) {
        if self.entity_lookup.contains_key(&entity) && !self.data.is_empty() {
            let swap_id = *self.entity_lookup.get(&entity).unwrap();
            let last_node_id = self.data.len() - 1;

            if last_node_id > 0 {
                // fix up the last node's hashmap value
                let entity_to_fix = self.data[last_node_id].entity_id();
                self.entity_lookup.insert(entity_to_fix, swap_id);
                // swap node to remove with last fixed up node then
                // remove the last item so that no items need to be shifted
                // aka one swap instead of potentially many
                self.data.swap(swap_id, last_node_id);
            }
            self.entity_lookup.remove(&entity);
            self.data.remove(last_node_id);
        }
    }
}
