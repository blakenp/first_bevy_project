mod camera;
mod asset_loader;
mod movement;
mod player;

use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
