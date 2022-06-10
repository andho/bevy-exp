use std::{fmt::Debug, hash::Hash};

use bevy::{
    math::Vec2,
    prelude::{Camera, CoreStage, Local, ParamSet, Plugin, Query, SystemSet, Transform, With},
};
use iyes_loopless::prelude::IntoConditionalSystem;

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

pub trait CameraState: Debug + Clone + Copy + PartialEq + Eq + Hash + Sync + Send {}

#[derive(Default)]
pub struct CameraPlugin<T: CameraState> {
    state: T,
}

impl<T: 'static + CameraState> CameraPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T: 'static + CameraState> Plugin for CameraPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(camera_movement.run_in_state(self.state));
    }
}
