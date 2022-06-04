mod animation;

use animation::{AnimationPlugin, SpriteSheetAnimation};
use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, OrthographicCameraBundle, Res, ResMut,
        SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
    },
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

const SPEED: f32 = 100.0;
const ANIMATION_FPS: u8 = 12;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let texture_handle = asset_server.load("character/character-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 11, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let anim_handle = animations.add(SpriteSheetAnimation::from_frames((1..11).collect(), 12));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(anim_handle);
}
