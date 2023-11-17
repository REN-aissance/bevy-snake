use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::extract_component::ExtractComponent,
};

const CAMERA_DISTANCE: f32 = 30.;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

#[derive(Component, Debug)]
pub struct AstCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        AstCamera,
    ));
}

fn move_camera(
    mut query: Query<&mut Transform, With<AstCamera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    if input.pressed(KeyCode::E) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::X, 0.5 * time.delta_seconds()),
        );
    } else if input.pressed(KeyCode::Q) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::X, -0.5 * time.delta_seconds()),
        );
    }
}
