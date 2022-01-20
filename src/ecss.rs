use crate::{Collection, Component, EntityCollection};
use std::{
    any::type_name,
    collections::{HashMap, HashSet},
};

pub type EntityId = usize;

#[macro_export]
macro_rules! component_types {
    ($($t:ident),+) => {
        #[derive(Debug)]
        pub enum ComponentType {
            $($t),+
        }

        $(impl Component for $t { //$w where specs
            fn get_type_id() -> u8 {
                ComponentType::$t as u8
            }

            fn get_entity_id(&self) -> usize {
                self.entity_id
            }
        })+

    };
}

pub struct ECSS {
    entity_counter: EntityId,
    reusable_entities: Vec<EntityId>,
    type_map: HashMap<u8, Box<dyn EntityCollection>>,
    entity_map: HashMap<EntityId, HashSet<u8>>,
}

impl Default for ECSS {
    fn default() -> Self {
        Self {
            entity_counter: std::usize::MAX,
            reusable_entities: Vec::new(),
            type_map: HashMap::new(),
            entity_map: HashMap::new(),
        }
    }
}

impl ECSS {
    fn get_collection<T>(&self) -> &Collection<T>
    where
        T: 'static + Component,
    {
        let type_id = T::get_type_id();
        if let Some(components) = self.type_map.get(&type_id) {
            return components.as_any().downcast_ref::<Collection<T>>().unwrap();
        }
        panic!(
            "The Type: {} has not been registered with this ECSS instance.",
            type_name::<T>()
        );
    }

    fn get_collection_mut<T>(&mut self) -> &mut Collection<T>
    where
        T: 'static + Component,
    {
        let type_id = T::get_type_id();
        if let Some(components) = self.type_map.get_mut(&type_id) {
            return components
                .as_any_mut()
                .downcast_mut::<Collection<T>>()
                .unwrap();
        }
        panic!(
            "The Type: {} has not been registered with this ECSS instance.",
            type_name::<T>()
        );
    }

    /// Returns all entities that have the specified type
    /// # Examples
    /// ```
    /// 
    /// ```
    pub fn entities_by_type<T>(&mut self) -> Vec<EntityId>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection::<T>();
        collection.get_entities()
    }


    pub fn components(&self, entity: EntityId) -> impl Iterator<Item = &u8> {
        self.entity_map.get(&entity).unwrap().into_iter()
    }

    pub fn create_entity(&mut self) -> EntityId {
        if !self.reusable_entities.is_empty() {
            return self.reusable_entities.pop().unwrap();
        }
        if self.entity_counter <= 0 {
            panic!("You've reached the max number of entities.");
        }
        let entity_id = self.entity_counter;
        self.entity_counter -= 1;
        self.entity_map.insert(entity_id, HashSet::new());
        entity_id
    }

    pub fn create<T>(&mut self, data: T)
    where
        T: 'static + Component,
    {
        let entity_id = data.get_entity_id();
        let collection = self.get_collection_mut();
        if !collection.contains(entity_id) {
            collection.create(data);
            drop(collection);

            let type_list = self.entity_map.get_mut(&entity_id).unwrap();
            type_list.insert(T::get_type_id());
        }
    }

    pub fn entities_where<T, F>(&self, f: F) -> Vec<EntityId>
    where
        T: 'static + Component,
        F: Fn(&T) -> bool + 'static,
    {
        let collection = self.get_collection();
        collection.entities_where(f)
    }

    pub fn exists<T>(&self, entity: EntityId) -> bool
    where
        T: 'static + Component,
    {
        let collection = self.get_collection::<T>();
        collection.contains(entity)
    }

    pub fn get<T>(&self, entity: EntityId) -> Option<&T>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection();
        collection.get(entity)
    }

    pub fn get_mut<T>(&mut self, entity: EntityId) -> Option<&mut T>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection_mut();
        collection.get_mut(entity)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = &T>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection();
        collection.iter()
    }

    pub fn iter_mut<T>(&mut self) -> impl Iterator<Item = &mut T>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection_mut();
        collection.iter_mut()
    }

    pub fn iter_with_entities_mut<T>(&mut self) -> impl Iterator<Item = (EntityId, &mut T)>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection_mut();
        collection.iter_with_entities_mut()
    }
    pub fn iter_with_entities<T>(&self) -> impl Iterator<Item = (EntityId, &T)>
    where
        T: 'static + Component,
    {
        let collection = self.get_collection();
        collection.iter_with_entities()
    }

    pub fn register<T>(&mut self)
    where
        T: 'static + Component,
    {
        let type_id = T::get_type_id();
        if !self.type_map.contains_key(&type_id) {
            self.type_map
                .insert(type_id, Box::new(Collection::<T>::default()));
        }
    }
    pub fn remove<T>(&mut self, entity: EntityId)
    where
        T: 'static + Component,
    {
        let collection = self.get_collection_mut::<T>();
        collection.remove(entity);
        let type_list = self.entity_map.get_mut(&entity).unwrap();
        type_list.remove(&T::get_type_id());
    }

    pub fn remove_all(&mut self, entity: EntityId) {
        for components in self.type_map.values_mut() {
            components.remove(entity);
        }
    }

    pub fn register_sized<T>(&mut self, size: usize)
    where
        T: 'static + Component,
    {
        let type_id = T::get_type_id();
        if !self.type_map.contains_key(&type_id) {
            self.type_map
                .insert(type_id, Box::new(Collection::<T>::new(size)));
        }
    }
}

#[test]
fn test() {
    component_types!(Position, AttachedTo);

    #[derive(Debug)]
    struct Position {
        entity_id: EntityId,
        test: u32,
    }
    #[derive(Debug)]
    struct AttachedTo {
        entity_id: EntityId,
        attached_to_entity: EntityId,
    }

    assert_eq!(Position::get_type_id(), 0);

    let mut ecs = ECSS::new();
    ecs.register_sized::<Position>(4);
    ecs.register_sized::<AttachedTo>(1);

    let entity_0 = ecs.create_entity();
    let entity_1 = ecs.create_entity();
    let entity_2 = ecs.create_entity();
    let entity_3 = ecs.create_entity();
    let entity_4 = ecs.create_entity();

    ecs.create(Position {
        entity_id: entity_0,
        test: 0,
    });

    ecs.create(Position {
        entity_id: entity_1,
        test: 1,
    });

    ecs.create(Position {
        entity_id: entity_2,
        test: 2,
    });

    ecs.create(Position {
        entity_id: entity_3,
        test: 3,
    });

    ecs.create(Position {
        entity_id: entity_4,
        test: 4,
    });

    for i in ecs.entities_where(move |e: &Position| e.test == 3) {
        assert!(i == entity_3);
    }

    ecs.create(AttachedTo {
        entity_id: entity_0,
        attached_to_entity: 3,
    });

    for typeid in ecs.components(entity_0) {
        let typename: ComponentType = unsafe { std::mem::transmute(*typeid) };
        println!("{:?}", typename);
    }

    assert!(ecs.exists::<Position>(entity_0));
    assert!(ecs.exists::<Position>(entity_1));
    assert!(ecs.exists::<Position>(entity_2));
    assert!(ecs.exists::<Position>(entity_3));

    assert_eq!(ecs.exists::<Position>(entity_4), false);

    {
        let entity_1_diff_mem: usize = entity_1.clone();
        ecs.remove::<Position>(entity_1_diff_mem);
    }

    assert!(ecs.get::<Position>(entity_1).is_none());

    ecs.remove::<Position>(entity_1);

    if let Some(pos) = ecs.get::<Position>(entity_3) {
        assert_eq!(pos.test, 3);
    } else {
        panic!()
    }

    let mut expected = vec![entity_2, entity_3, entity_0];
    for (entity, _item) in ecs.iter_with_entities::<Position>() {
        assert_eq!(entity, expected.pop().unwrap());
    }

    let mut expected = vec![entity_0, entity_2, entity_3];
    let mut entities = ecs.entities_by_type::<Position>();
    entities.sort();
    for entity in entities {
        assert_eq!(entity, expected.pop().unwrap())
    }

    ecs.create(Position {
        entity_id: entity_4,
        test: 0,
    });

    assert!(ecs.exists::<Position>(entity_4));

    if let Some(pos) = ecs.get_mut::<Position>(entity_4) {
        pos.test = 4;
    };

    ecs.remove::<Position>(entity_2);
    ecs.remove_all(entity_0);

    assert!(!ecs.exists::<Position>(entity_0));
    assert!(!ecs.exists::<AttachedTo>(entity_0));

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

    assert_eq!(ecs.iter::<Position>().count(), 0)
}
