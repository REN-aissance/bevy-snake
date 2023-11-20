use bevy::{prelude::*, utils::HashMap};

use crate::{
    fruit::Fruit,
    snek::{Segment, SnekHead},
};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_event::<FruitEatenEvent>()
            .add_systems(Update, collision_detection)
            .add_systems(FixedUpdate, fruit_collision)
            .add_systems(FixedUpdate, snek_collision);
    }
}

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Debug, Component, Default)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(r: f32) -> Collider {
        Collider {
            radius: r / 2.0,
            colliding_entities: vec![],
        }
    }
}

fn collision_detection(mut q: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for (entity_a, transform_a, collider_a) in q.iter() {
        for (entity_b, transform_b, collider_b) in q.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance_squared(transform_b.translation());
                info!(
                    "{:?}, {:?}",
                    distance,
                    (collider_a.radius + collider_b.radius).powi(2)
                );
                if distance < (collider_a.radius + collider_b.radius).powi(2) {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in q.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn snek_collision(
    mut commands: Commands,
    q: Query<&Collider, With<Segment>>,
    snekhead: Query<Entity, With<SnekHead>>,
) {
    q.iter().for_each(|collider| {
        collider.colliding_entities.iter().for_each(|&e| {
            if let Ok(e) = snekhead.get(e) {
                commands.entity(e).despawn();
            }
        })
    });
}

#[derive(Event)]
pub struct FruitEatenEvent;

fn fruit_collision(
    mut commands: Commands,
    q: Query<(Entity, &Collider), With<Fruit>>,
    snekhead: Query<Entity, With<SnekHead>>,
    mut er: EventWriter<FruitEatenEvent>,
) {
    let mut events = vec![];
    q.iter().for_each(|(fruit, collider)| {
        collider.colliding_entities.iter().for_each(|&e| {
            if snekhead.contains(e) {
                commands.entity(fruit).despawn();
                events.push(FruitEatenEvent);
            }
        })
    });
    er.send_batch(events);
}
