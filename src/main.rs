mod fixed_timestep;
mod fruit;
mod movement;
mod snek;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::ScalingMode,
    window::{close_on_esc, WindowResolution},
};
use fixed_timestep::FixedTimestepPlugin;
use fruit::FruitPlugin;
use movement::MovementPlugin;
use snek::SnekPlugin;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Snek Game"),
                        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, add_camera)
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, add_rand)
        .add_systems(Startup, add_background)
        .add_plugins(FixedTimestepPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SnekPlugin)
        .add_plugins(FruitPlugin)
        .run();
}

fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(600.0);
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);
    commands.spawn(camera);
}

fn add_rand(world: &mut World) {
    world.insert_non_send_resource(rand::thread_rng());
}

fn add_background(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.17, 0.17, 0.17),
            custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    });
}
