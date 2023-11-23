use std::time::Duration;

use bevy::prelude::*;

use crate::{
    fixed_timestep::{FixedTick, PostFixedTick},
    fruit::{Fruit, FruitEatenEvent},
    movement::{MovementBundle, MovementEvent, Velocity},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub const STEP_SIZE: f32 = 20.;
pub const PADDING: f32 = 2.5;

pub struct SnekPlugin;
impl Plugin for SnekPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Direction::None)
            .add_systems(Startup, spawn_snek)
            .add_systems(Update, handle_snek_input)
            .add_systems(
                FixedTick,
                (update_last_dir, add_child, move_children, fruit_collision).chain(),
            )
            .add_systems(Update, animate_snek)
            .add_systems(PostFixedTick, self_collision)
            .add_systems(PostFixedTick, wall_collision);
    }
}

#[derive(Component, Resource, Default, Eq, PartialEq, Copy, Clone)]
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
    pub animation: f32,
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
    ));
}

fn handle_snek_input(
    input: Res<Input<KeyCode>>,
    mut q: Query<(&mut Velocity, &mut SnekHead)>,
    last_dir: Res<Direction>,
) {
    let (mut v, mut snek_head) = q.get_single_mut().unwrap();
    for &e in input.get_pressed() {
        if e == KeyCode::W && *last_dir != Direction::Down {
            snek_head.dir = Direction::Up;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::S && *last_dir != Direction::Up {
            snek_head.dir = Direction::Down;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::A && *last_dir != Direction::Right {
            snek_head.dir = Direction::Left;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
        if e == KeyCode::D && *last_dir != Direction::Left {
            snek_head.dir = Direction::Right;
            v.0 = snek_head.dir.val() * STEP_SIZE;
        }
    }
}

fn move_children(
    test: Query<Entity, (With<SnekHead>, Without<Segment>)>,
    snek: Query<&Velocity, (With<SnekHead>, Without<Segment>)>,
    mut q: Query<&mut Velocity, With<Segment>>,
    mut er: EventReader<MovementEvent>,
) {
    for event in er.read() {
        if test.contains(event.0) {
            let mut prev = *snek.single();
            q.iter_mut().for_each(|mut e| {
                std::mem::swap(&mut *e, &mut prev);
            });
        }
    }
}

fn update_last_dir(q: Query<&SnekHead>, mut last_dir: ResMut<Direction>) {
    *last_dir = q.single().dir;
}

fn add_child(
    mut commands: Commands,
    mut snek: Query<(&Transform, &mut SnekHead), Without<Segment>>,
    tail: Query<&Transform, With<Segment>>,
    mut er: EventReader<FruitEatenEvent>,
    mut fixed_time: ResMut<Time<Fixed>>,
) {
    if er.read().next().is_some() {
        *fixed_time = Time::from_duration(fixed_time.delta().mul_f32(0.9978));
        let (mut t, mut snek) = snek.single_mut();
        if !snek.children.is_empty() {
            t = tail.get(snek.children[snek.children.len() - 1]).unwrap();
        }
        let id = commands
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
            .id();
        snek.children.push(id);
    }
}

fn fruit_collision(
    mut commands: Commands,
    q: Query<(Entity, &Transform), With<Fruit>>,
    s: Query<&Transform, (With<SnekHead>, Without<Fruit>)>,
    mut ew: EventWriter<FruitEatenEvent>,
) {
    let snake_transform = s.single();
    let mut events = vec![];
    q.iter().for_each(|(e, t)| {
        if t.translation.distance_squared(snake_transform.translation) < STEP_SIZE.powi(2) {
            events.push(FruitEatenEvent);
            commands.get_entity(e).unwrap().despawn();
        }
    });
    ew.send_batch(events);
}

fn self_collision(
    snek: Query<(&Transform, &SnekHead)>,
    segments: Query<&Transform, With<Segment>>,
    mut fixed_time: ResMut<Time<Fixed>>,
) {
    //First 3 should be impossible to collide with
    for seg in snek.single().1.children.iter().skip(3) {
        let t = segments.get(*seg).unwrap();
        if snek.single().0.translation.distance_squared(t.translation) < STEP_SIZE.powi(2) {
            *fixed_time = Time::from_duration(Duration::from_secs(999999));
        }
    }
}

fn wall_collision(snek: Query<&Transform, With<SnekHead>>, mut fixed_time: ResMut<Time<Fixed>>) {
    let t = snek.single().translation;
    if t.x < -SCREEN_WIDTH / 2.0 || t.x > SCREEN_WIDTH / 2.0 {
        *fixed_time = Time::from_duration(Duration::from_secs(999999));
    }
    if t.y < -SCREEN_HEIGHT / 2.0 || t.y > SCREEN_HEIGHT / 2.0 {
        *fixed_time = Time::from_duration(Duration::from_secs(999999));
    }
}

fn animate_snek(
    mut q: Query<&mut Transform, With<Segment>>,
    snek: Query<&SnekHead>,
    time: Res<Time>,
) {
    snek.single()
        .children
        .iter()
        .enumerate()
        .for_each(|(i, seg)| {
            let mut t = q.get_mut(*seg).unwrap();
            t.scale = Vec3::splat((0.5 * time.elapsed_seconds() + i as f32).sin() / 8.0 + 0.75)
        });
}
