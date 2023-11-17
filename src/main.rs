mod asset_loader;
mod asteroid;
mod camera;
mod debug;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use bevy::{
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Spaceship".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, spawn_point_light)
        .add_plugins(AssetLoaderPlugin)
        .add_systems(Update, close_on_esc)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(DebugPlugin)
        .run()
}

fn spawn_point_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-100.0, 100.0, 0.0),
        point_light: PointLight {
            intensity: 3000000.0,
            range: 10_000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
