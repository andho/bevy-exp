use bevy::{
    math::Vec2,
    prelude::{Camera, CoreStage, Local, ParamSet, Plugin, Query, SystemSet, Transform, With},
};

use crate::input::Player;

fn camera_movement(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
    mut player_position: Local<Vec2>,
) {
    for player_transform in transforms.p1().iter() {
        *player_position = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
    }

    for mut camera_transform in transforms.p0().iter_mut() {
        *camera_transform = Transform::from_xyz(player_position.x, player_position.y, 0.0);
    }
}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(camera_movement),
        );
    }
}
