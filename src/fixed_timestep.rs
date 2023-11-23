use std::time::Duration;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

const STARTING_DELAY: Duration = Duration::from_millis(150);

pub struct FixedTimestepPlugin;
impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(PreFixedTick)
            .init_schedule(FixedTick)
            .init_schedule(PostFixedTick)
            .add_systems(Update, update_last_time)
            .add_systems(FixedTick, update_frame)
            .insert_resource(Time::<Fixed>::from_duration(STARTING_DELAY))
            .add_systems(FixedUpdate, run_fixed_tick);
    }
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

fn update_last_time(mut last_time: Local<f32>, time: Res<Time>) {
    *last_time = time.elapsed_seconds();
}

fn update_frame(mut last_time: Local<f32>, fixed_time: Res<Time<Fixed>>) {
    *last_time = fixed_time.elapsed_seconds();
}
