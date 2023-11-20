mod fixed_timestep;
mod fruit;
mod movement;
mod snek;

use bevy::{
    ecs::schedule::{ExecutorKind, ScheduleLabel},
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use fixed_timestep::FixedTimestepPlugin;
use fruit::FruitPlugin;
use movement::MovementPlugin;
use snek::SnekPlugin;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    let mut schedule = Schedule::default();
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Snek Game"),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_schedule(schedule)
        .init_schedule(PreFixedTick)
        .init_schedule(FixedTick)
        .init_schedule(PostFixedTick)
        .add_systems(FixedUpdate, run_fixed_tick)
        .add_systems(Startup, add_camera)
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, add_rand)
        .add_plugins(FixedTimestepPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SnekPlugin)
        .add_plugins(FruitPlugin)
        .run();
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreFixedTick;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixedTick;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostFixedTick;

pub fn run_fixed_tick(world: &mut World) {
    world.run_schedule(PreFixedTick);
    world.run_schedule(FixedTick);
    world.run_schedule(PostFixedTick);
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_rand(world: &mut World) {
    world.insert_non_send_resource(rand::thread_rng());
}
