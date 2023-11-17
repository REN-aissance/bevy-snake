use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec3 = Vec3::ZERO;
const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_VELOCITY: Vec3 = Vec3::ZERO;
const SPEED: f32 = 1000.;
const ROTATION_SPEED: f32 = 5.;
const ROLL_SPEED: f32 = 5.;
const MISSILE_SPEED: f32 = 50.;
const MISSILE_ACCELERATION: f32 = 20.;
const MISSILE_FORWARD: f32 = 8.5;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement)
            .add_systems(Update, spaceship_weapon_controls);
    }
}

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(STARTING_VELOCITY),
            acceleration: Acceleration(STARTING_ACCELERATION),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut roll = 0.0;
    let mut rotation = 0.0;
    let mut movement = 0.0;

    if input.pressed(KeyCode::W) {
        movement = SPEED * time.delta_seconds();
    } else if input.pressed(KeyCode::S) {
        movement = -SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if input.pressed(KeyCode::D) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::J) {
        roll = -ROLL_SPEED * time.delta_seconds();
    } else if input.pressed(KeyCode::K) {
        roll = ROLL_SPEED * time.delta_seconds();
    }

    velocity.0 = -transform.forward() * movement;
    transform.rotate_local_z(roll);
    transform.rotate_y(rotation);
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
    input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    if input.pressed(KeyCode::Space) {
        let (transform, mut velocity) = query.single_mut();
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration(-transform.forward() * MISSILE_ACCELERATION),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD,
                    )
                    .with_scale(Vec3::new(0.5, 0.5, 0.5))
                    .with_rotation(transform.rotation),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
