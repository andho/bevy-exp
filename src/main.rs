mod animation;

use bevy::{
    core::Time,
    prelude::{
        AddAsset, App, AssetServer, Assets, Commands, Entity, Handle, OrthographicCameraBundle,
        Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3, With, Without,
    },
    sprite::TextureAtlasSprite,
    DefaultPlugins,
};

use crate::animation::{SpriteSheetAnimation, SpriteSheetAnimationState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<SpriteSheetAnimation>()
        .add_startup_system(setup)
        .add_system(add_animation_state)
        .add_system(animate)
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

fn add_animation_state(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<Handle<SpriteSheetAnimation>>,
            Without<SpriteSheetAnimationState>,
        ),
    >,
) {
    for entity in query.iter() {
        println!("found some entitiet without animation state");
        commands
            .entity(entity)
            .insert(SpriteSheetAnimationState::default());
    }
}

fn animate(
    time: Res<Time>,
    animation_defs: Res<Assets<SpriteSheetAnimation>>,
    mut animations: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &Handle<SpriteSheetAnimation>,
        &mut SpriteSheetAnimationState,
    )>,
) {
    for (entity, sprite, animation, mut state) in
        animations
            .iter_mut()
            .filter_map(|(entity, sprite, anim_handle, state)| {
                animation_defs
                    .get(anim_handle)
                    .map(|anim| (entity, sprite, anim, state))
            })
    {
        state.update(&time, sprite, animation);
    }
}
