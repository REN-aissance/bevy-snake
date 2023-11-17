use std::{f32::consts::TAU, ops::Range};

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const VELOCITY_SCALAR: f32 = 10.;
const ACCELERATION_SCALAR: f32 = 1.;
const SPAWN_RANGE_X: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Z: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Y: Range<f32> = -5.0..5.0;
//Speed in radians per second
const ANGULAR_SPEED: f32 = TAU;
const SPAWN_TIME: f32 = 0.3;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_TIME,
            TimerMode::Repeating,
        )))
        .add_systems(Update, spawn_asteroid);
    }
}

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer(Timer);

fn spawn_asteroid(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    scene_assets: Res<SceneAssets>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        rng.gen_range(SPAWN_RANGE_Y),
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize();

    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(velocity),
            acceleration: Acceleration(acceleration),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation).with_scale(Vec3::splat(0.7)),
                ..default()
            },
        },
        Asteroid,
    ));
}
