mod animation;
mod animator;

use core::fmt;

use animation::{AnimationPlugin, SpriteSheetAnimation};
use animator::{animation_selection, AnimationKey, Animator};
use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    prelude::{
        App, AssetServer, Assets, Commands, Component, OrthographicCameraBundle, Res, ResMut,
        SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
    },
    utils::HashMap,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(animation_selection::<Animations, AnimationData>)
        .run();
}

//const SPEED: f32 = 100.0;
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

#[derive(Component, Clone)]
struct AnimationData;

fn animation_selector(_data: AnimationData) -> Animations {
    Animations::Walk
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
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(animator)
        .insert(AnimationData {});
}
