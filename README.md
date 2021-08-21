# ecss
Stupid simple ECS written in Rust.

# Usage
```
#[derive(Debug)]
struct Transform {
    position: [f32;4],
}

#[derive(Debug)]
struct Collision {
    rect: Rect,
}

// register struct types as components,
// allows type id's to be accessed with T::get_type_id();
component_types!(
    Transform,
    Collision,
);

// create the ecss instance
let mut ecs = ECSS::new();

// allocates enough for 25 components of each type
ecs.register_sized::<Transform>(25);
ecs.register_sized::<Collision>(25);

// generate some entities
let entity_0 = ecs.create_entity();
let entity_1 = ecs.create_entity();
let entity_2 = ecs.create_entity();
let entity_3 = ecs.create_entity();

ecs.create(entity_0, Transform { position: [0.0; 4] });
ecs.create(entity_0, Collision { rect: Rect::new() });

// query entities by type based on condition
for entity in ecs.entities_where(move |e: &Transform| e.position == [0.0 as f32; 4]) {
    println!("Found Entity: [{:?}]", entity);
}

// iterate through data by type, along with entity_id
for (entity, _item) in ecs.iter_with_entities::<Transform>() {
    ...
}

// iterate through just data
for item in ecs.iter_mut::<Transform>() {
    item.position[0] += 5.0;
    item.position[1] += 4.0;
    item.position[2] += 3.0;
    item.position[3] += 2.0;
}

assert!(ecs.exists::<Transform>(entity_0));
assert!(ecs.get::<Transform>(entity_1).is_none());

// access single item
if let Some(transform) = ecs.get_mut::<Transform>(entity_0) {
    transform.position[0] = 2.0;
    transform.position[1] = 4.0;
};

// remove components for a given entity
ecs.remove::<Transform>(entity_0);
ecs.remove::<Collision>(entity_0);
```