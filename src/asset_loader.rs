use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *assets = SceneAssets {
        asteroid: asset_server.load("My_Planet.glb#Scene0"),
        spaceship: asset_server.load("Spaceship_Green.glb#Scene0"),
        missiles: asset_server.load("Missiles.glb#Scene0"),
    }
}
