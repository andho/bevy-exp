use crate::{
    animation::{AnimationPlugin, AnimationState, SpriteSheetAnimation},
    animator::{animation_selection, AnimationKey, Animator},
    camera::{CameraPlugin, CameraState},
    input::{MovementPlugin, MovementState, Player, Velocity},
    loading::{LoadingPlugin, LoadingState1},
    mouse::{MousePlugin, MouseState},
};
use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, OrthographicCameraBundle, Plugin, Query, Res,
        ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
    },
    sprite::SpriteBundle,
    utils::HashMap,
};
use core::fmt;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem};
use iyes_progress::ProgressPlugin;

const ANIMATION_FPS: u8 = 12;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Animations {
    Idle,
    Walk,
}

impl fmt::Display for Animations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AnimationKey for Animations {}

impl Default for Animations {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component, Clone, Default, Debug)]
struct AnimationData {
    moving: bool,
}

fn animation_selector(data: AnimationData) -> Animations {
    match data.moving {
        true => Animations::Walk,
        false => Animations::Idle,
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let texture_handle = asset_server.load("character/character-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 11, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let anim_idle_handle =
        animations.add(SpriteSheetAnimation::from_frames(vec![0], ANIMATION_FPS));
    let anim_walk_handle = animations.add(SpriteSheetAnimation::from_frames(
        (1..11).collect(),
        ANIMATION_FPS,
    ));

    let animator = Animator::new(
        HashMap::from_iter([
            (Animations::Idle, anim_idle_handle),
            (Animations::Walk, anim_walk_handle),
        ]),
        animation_selector,
    );

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("background/cracked-dirt.png"),
        transform: Transform::from_scale(Vec3::splat(10.0)),
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(animator)
        .insert(AnimationData::default())
        .insert(Player {});
}

fn update_animation_data(mut query: Query<(&Velocity, &mut AnimationData)>) {
    for (velocity, mut anim_data) in query.iter_mut() {
        if velocity.length() > 0.0 {
            anim_data.moving = true;
        } else {
            anim_data.moving = false;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    InGame,
}

impl AnimationState for GameState {}
impl MovementState for GameState {}
impl CameraState for GameState {}
impl MouseState for GameState {}
impl LoadingState1 for GameState {}

#[derive(Default)]
pub struct GooMainPlugin;

impl Plugin for GooMainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(GameState::Loading)
            .add_plugin(LoadingPlugin::new(GameState::Loading, GameState::InGame))
            .add_plugin(AnimationPlugin::new(GameState::InGame))
            .add_plugin(MovementPlugin::new(GameState::InGame))
            .add_plugin(CameraPlugin::new(GameState::InGame))
            .add_plugin(MousePlugin::new(GameState::InGame))
            .add_enter_system(GameState::InGame, setup)
            .add_system(
                animation_selection::<Animations, AnimationData>.run_in_state(GameState::InGame),
            )
            .add_system(update_animation_data.run_in_state(GameState::InGame));
    }
}
