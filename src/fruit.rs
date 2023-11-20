use std::ops::Range;

use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{snek::STEP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
const PADDING: f32 = 7.5;
const SPAWN_RANGE_X: Range<i32> =
    ((-SCREEN_WIDTH / STEP_SIZE / 2.0) as i32)..((SCREEN_WIDTH / STEP_SIZE / 2.0) as i32);
const SPAWN_RANGE_Y: Range<i32> =
    ((-SCREEN_HEIGHT / STEP_SIZE / 2.0) as i32)..((SCREEN_HEIGHT / STEP_SIZE / 2.0) as i32);

pub struct FruitPlugin;
impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<FruitEatenEvent>>()
            .add_systems(Update, spawn_fruit)
            .add_systems(Update, animate_fruit);
    }
}

#[derive(Component)]
pub struct Fruit;

#[derive(Event)]
pub struct FruitEatenEvent;

fn spawn_fruit(input: Res<Input<KeyCode>>, mut commands: Commands, mut rng: NonSendMut<ThreadRng>) {
    if input.pressed(KeyCode::Space) {
        let x = rng.gen_range(SPAWN_RANGE_X) as f32 * STEP_SIZE + STEP_SIZE / 2.0;
        let y = rng.gen_range(SPAWN_RANGE_Y) as f32 * STEP_SIZE + STEP_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(STEP_SIZE - PADDING)),
                    color: Color::TOMATO,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            Fruit,
        ));
    }
}

fn animate_fruit(mut q: Query<&mut Transform, With<Fruit>>, time: Res<Time>) {
    for (i, mut t) in q.iter_mut().enumerate() {
        t.scale = Vec3::splat((2.0 * time.elapsed_seconds() + i as f32).sin() / 4.0 + 0.75);
    }
}
