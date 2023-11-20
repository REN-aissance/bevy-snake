use std::ops::Add;

use bevy::prelude::*;

use crate::{
    collision::{Collider, FruitEatenEvent},
    movement::{MovementBundle, MovementEvent, MovementPlugin, Velocity},
};

pub const STEP_SIZE: f32 = 20.;
pub const PADDING: f32 = 2.5;

pub struct SnekPlugin;
impl Plugin for SnekPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin)
            .add_systems(Startup, spawn_snek)
            .add_systems(Update, handle_snek_input)
            .add_systems(FixedUpdate, move_children)
            .add_systems(FixedUpdate, add_child);
    }
}

#[derive(Component, Default, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn val(&self) -> Vec3 {
        match self {
            Direction::Left => -Vec3::X,
            Direction::Right => Vec3::X,
            Direction::Up => Vec3::Y,
            Direction::Down => -Vec3::Y,
            _ => Vec3::ZERO,
        }
    }
}

#[derive(Component, Default)]
pub struct SnekHead {
    pub dir: Direction,
    pub children: Vec<Entity>,
}

#[derive(Event)]
pub struct SnekMovementEvent;

#[derive(Component)]
pub struct Segment;

fn spawn_snek(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                STEP_SIZE / 2.0,
                STEP_SIZE / 2.0,
                1.0,
            )),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(STEP_SIZE - PADDING)),
                color: Color::SEA_GREEN,
                ..default()
            },
            ..default()
        },
        MovementBundle::default(),
        SnekHead::default(),
        Collider::new(STEP_SIZE),
    ));
}

fn handle_snek_input(input: Res<Input<KeyCode>>, mut q: Query<(&mut Velocity, &mut SnekHead)>) {
    let (mut v, mut snek_head) = q.get_single_mut().unwrap();
    for &e in input.get_pressed() {
        if e == KeyCode::W && snek_head.dir != Direction::Down {
            snek_head.dir = Direction::Up;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::S && snek_head.dir != Direction::Up {
            snek_head.dir = Direction::Down;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::A && snek_head.dir != Direction::Right {
            snek_head.dir = Direction::Left;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::D && snek_head.dir != Direction::Left {
            snek_head.dir = Direction::Right;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
    }
}

fn move_children(
    snek: Query<(&Velocity, &SnekHead), Without<Segment>>,
    mut q: Query<&mut Velocity, With<Segment>>,
    mut er: EventReader<MovementEvent>,
) {
    for e in er.read() {
        if snek.contains(e.0) {
            let mut prev = *snek.single().0;
            for mut e in q.iter_mut() {
                std::mem::swap(&mut *e, &mut prev);
            }
        }
    }
}

fn add_child(
    mut commands: Commands,
    mut snek: Query<(&Transform, &mut SnekHead), Without<Segment>>,
    tail: Query<&Transform, With<Segment>>,
    mut er: EventReader<FruitEatenEvent>,
) {
    for _ in er.read() {
        let (mut t, mut snek) = snek.single_mut();
        if !snek.children.is_empty() {
            t = tail
                .get_component(snek.children[snek.children.len() - 1])
                .unwrap();
        }
        let id = if snek.children.len() > 4 {
            commands
                .spawn((
                    SpriteBundle {
                        transform: *t,
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(STEP_SIZE - PADDING)),
                            color: Color::SEA_GREEN,
                            ..default()
                        },
                        ..default()
                    },
                    MovementBundle::default(),
                    Segment,
                    Collider::new(STEP_SIZE),
                ))
                .id()
        } else {
            commands
                .spawn((
                    SpriteBundle {
                        transform: *t,
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(STEP_SIZE - PADDING)),
                            color: Color::SEA_GREEN,
                            ..default()
                        },
                        ..default()
                    },
                    MovementBundle::default(),
                    Segment,
                ))
                .id()
        };
        snek.children.push(id);
    }
}
