use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const SPRITESHEET_COLS: usize = 7;
const SPRITESHEET_ROWS: usize = 8;

const SPRITE_TILE_WIDTH: f32 = 128.0;
const SPRITE_TILE_HEIGHT: f32 = 256.0;

const SPRITE_IDX_STAND: usize = 28;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const PLAYER_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player).add_systems(
            Update,
            player_movement_controls,
        );
    }
}

fn spawn_player(mut commands: Commands, mut atlas: ResMut<Assets<TextureAtlas>>, scene_assets: Res<SceneAssets>) {
    let image_handle: Handle<Image> = scene_assets.player_image_handle.clone();
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let atlas_handle = atlas.add(texture_atlas);

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::cuboid(0.5, 0.5), // Adjust size as needed
            model: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(SPRITE_IDX_STAND), // Idle animation
                texture_atlas: atlas_handle,
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Player, // Add Player component
        Direction::Right, // Initial direction
    ));
}

fn player_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        velocity.value = Vec3::new(-PLAYER_SPEED * time.delta_seconds(), 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::D) {
        velocity.value = Vec3::new(PLAYER_SPEED * time.delta_seconds(), 0.0, 0.0);
    } else {
        velocity.value = Vec3::ZERO;
    }

    // Update the player's position based on velocity
    transform.translation += velocity.value;
}