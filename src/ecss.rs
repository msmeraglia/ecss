use std::{
    any::type_name,
    any::{Any, TypeId}, // TODO: Remove once proc macro generates type ids
    collections::HashMap,
};

use crate::{Collection, Component, Entity, INVALID_ENTITY};

// Stands for: ECS, Stupid.
pub struct ECSS {
    entities: Entity,
    reusable_entities: Vec<Entity>,
    components: Vec<Box<dyn Any>>,
    type_map: HashMap<TypeId, usize>, // type_id -> components vec index lookup
                                      // type_ids: Vec<TypeId>,Maybe use this to be able to iterate through all components/would need to probably implement a trait
}

//TODO Should eventually be generic, to allow client to pass in custom allocators now that Vec supports this.
impl Default for ECSS {
    fn default() -> Self {
        Self {
            entities: INVALID_ENTITY,
            reusable_entities: Vec::new(),
            components: Vec::new(),
            type_map: HashMap::new(), // K: type_id -> V: components vec index lookup
                                      // type_ids: Vec::new(),
        }
    }
}

impl ECSS {
    fn get_collection<T: 'static + Component + Sized>(&self) -> &Collection<T> {
        let type_id = TypeId::of::<T>();
        if let Some(index) = self.type_map.get(&type_id) {
            if let Some(components) = self.components.get(*index) {
                return components.downcast_ref::<Collection<T>>().unwrap();
            }
        }
        panic!(
            "The Type: {} has not been registered with this ECSS instance.",
            type_name::<T>()
        );
    }

    fn get_collection_mut<T: 'static + Component + Sized>(&mut self) -> &mut Collection<T> {
        let type_id = TypeId::of::<T>(); //TODO: Generate type id using proc macro to avoid this costly reflection
        if let Some(index) = self.type_map.get(&type_id) {
            if let Some(components) = self.components.get_mut(*index) {
                return components.downcast_mut::<Collection<T>>().unwrap();
            }
        }
        panic!(
            "The Type: {} has not been registered with this ECSS instance.",
            type_name::<T>()
        );
    }

    /// Returns all entities that have the specified type
    pub fn entities_by_type<T: 'static + Component + Sized>(&mut self) -> Vec<Entity> {
        let collection = self.get_collection::<T>();
        collection.get_entities()
    }

    pub fn components<T: 'static + Component + Sized>(&self) -> impl Iterator<Item = &T> {
        let collection = self.get_collection();
        collection.iter()
    }

    pub fn components_mut<T: 'static + Component + Sized>(
        &mut self,
    ) -> impl Iterator<Item = &mut T> {
        let collection = self.get_collection_mut();
        collection.iter_mut()
    }

    pub fn create_entity(&mut self) -> Entity {
        if !self.reusable_entities.is_empty() {
            return self.reusable_entities.pop().unwrap();
        }
        if self.entities < std::usize::MAX {
            self.entities += 1;
            return self.entities;
        }
        panic!("You've reached the max number of entities.");
    }

    // TODO: Return Result<> eventually for safer use, although usage will be more verbose...
    pub fn create<T: 'static + Component + Sized>(&mut self, data: T) {
        let collection = self.get_collection_mut();
        if !collection.contains(data.get_entity()) {
            return collection.create(data);
        }
    }

    pub fn exists<T: 'static + Component + Sized>(&self, entity: Entity) -> bool {
        let collection = self.get_collection::<T>();
        collection.contains(entity)
    }

    pub fn get<T: 'static + Component + Sized>(&self, entity: Entity) -> Option<&T> {
        let collection = self.get_collection();
        collection.get(entity)
    }

    pub fn get_mut<T: 'static + Component + Sized>(&mut self, entity: Entity) -> Option<&mut T> {
        let collection = self.get_collection_mut();
        collection.get_mut(entity)
    }
    pub fn new() -> Self {
        Default::default()
    }

    // Hmmm, Should this be allowed, due to potential for reallocation?
    pub fn register<T: 'static + Component + Sized>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.type_map.contains_key(&type_id) {
            self.type_map.insert(type_id, self.components.len());
            self.components.push(Box::new(Collection::<T>::default()));
        }
    }
    pub fn remove<T: 'static + Component + Sized>(&mut self, entity: Entity) {
        let collection = self.get_collection_mut::<T>();
        collection.remove(entity);
    }

    pub fn register_sized<T: 'static + Component + Sized>(&mut self, size: usize) {
        let type_id = TypeId::of::<T>();
        if self.type_map.contains_key(&type_id) {
            // TODO!: Check to see if capacity is less than input size, id so do with_capacity,
            // but make sure its empty, otherwise this could be costly if already filled with items
        } else {
            self.type_map.insert(type_id, self.components.len());
            self.components.push(Box::new(Collection::<T>::new(size)));
        }
    }
}


#[test]
fn test() {
    #[derive(Component, Debug)]
    struct Position {
        entity_id: Entity,
        test: u32,
    }
    // Check to test component derive is working
    let position = Position { entity_id: 2, test: 3 };
    assert_eq!(position.get_entity(), 2);

    let mut ecs = ECSS::new();
    ecs.register_sized::<Position>(4);

    let entity_0 = ecs.create_entity();
    let entity_1 = ecs.create_entity();
    let entity_2 = ecs.create_entity();
    let entity_3 = ecs.create_entity();
    let entity_4 = ecs.create_entity();

    ecs.create(Position{
         entity_id: entity_0,
         test:0
    });

    ecs.create(Position{
        entity_id: entity_1,
        test: 1
    });

    ecs.create(Position{
        entity_id: entity_2,
        test: 2
    });

    ecs.create(Position{
        entity_id: entity_3,
        test: 3
    });

    ecs.create(Position{
        entity_id: entity_4,
        test: 4
    });

    assert!(ecs.exists::<Position>(entity_0));
    assert!(ecs.exists::<Position>(entity_1));
    assert!(ecs.exists::<Position>(entity_2));
    assert!(ecs.exists::<Position>(entity_3));
    assert_eq!(ecs.exists::<Position>(entity_4), false);

    ecs.remove::<Position>(entity_1);
    ecs.remove::<Position>(entity_1); // Should just do nothing, but pass

    // Free == 1, Slots[1] = 4
    if let Some(pos) = ecs.get::<Position>(entity_3) {
        assert_eq!(pos.test, 3);
    } else {
        panic!()
    }

    let mut expected = vec![3, 4, 1];
    for item in ecs.components::<Position>() {
        assert_eq!(item.entity_id, expected.pop().unwrap());
    }

    expected = vec![4, 3, 1];
    let mut entities = ecs.entities_by_type::<Position>();
    entities.sort();
    for entity in entities {
        assert_eq!(entity, expected.pop().unwrap());
    }

    ecs.create(Position{
        entity_id: entity_4,
        test: 0
    });

    assert!(ecs.exists::<Position>(entity_4));

    if let Some(pos) = ecs.get_mut::<Position>(entity_4) {
        pos.test = 4;
    };

    ecs.remove::<Position>(entity_2);
    ecs.remove::<Position>(entity_0);

    if let Some(pos) = ecs.get::<Position>(entity_4) {
        assert_eq!(pos.test, 4);
    } else {
        panic!()
    }

    ecs.remove::<Position>(entity_4);

    if let Some(pos) = ecs.get::<Position>(entity_3) {
        assert_eq!(pos.test, 3);
    } else {
        panic!()
    }

    ecs.remove::<Position>(entity_3);

    assert_eq!(ecs.components::<Position>().count(), 0)
}
