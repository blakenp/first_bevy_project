use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const PLAYER_SPEED: f32 = 25.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player).add_systems(
            Update,
            player_movement_controls,
        );
    }
}

fn spawn_player(mut commands: Commands, scene_assests: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::cuboid(0.5, 0.5),
            model: SceneBundle {
                scene: scene_assests.player_texture_atlas_sprite.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Player,
    ));
}

fn player_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = query.single_mut();
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        movement = -PLAYER_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        movement = PLAYER_SPEED * time.delta_seconds();
    }

    // Update the players's velocity based on new direction.
    velocity.value = -transform.forward() * movement;
}