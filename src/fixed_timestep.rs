use std::time::Duration;

use bevy::prelude::*;

const STARTING_DELAY: Duration = Duration::from_millis(150);

pub struct FixedTimestepPlugin;
impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_last_time)
            .add_systems(FixedUpdate, update_frame)
            .insert_resource(Time::<Fixed>::from_duration(STARTING_DELAY));
    }
}

fn update_last_time(mut last_time: Local<f32>, time: Res<Time>) {
    *last_time = time.elapsed_seconds();
}

fn update_frame(mut last_time: Local<f32>, fixed_time: Res<Time<Fixed>>) {
    *last_time = fixed_time.elapsed_seconds();
}
