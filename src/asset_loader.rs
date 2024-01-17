use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player_image_handle: Handle<Image>, // Store a Handle<Image>
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("spritesheets/spritesheet_players.png");
    scene_assets.player_image_handle = image_handle; // Now holds a Handle<Image>
}
