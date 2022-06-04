use bevy::core::Timer;
use bevy::ecs::component::Component;
use bevy::prelude::{CoreStage, Plugin, SystemSet};
use bevy::reflect::TypeUuid;
use bevy::sprite::TextureAtlasSprite;
use bevy::{
    core::Time,
    prelude::{AddAsset, App, Assets, Commands, Entity, Handle, Query, Res, Without},
};
use std::ops::DerefMut;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "14069b02-588b-4fdf-be17-60d158301129"]
pub struct SpriteSheetAnimation {
    frames: Vec<usize>,
    fps: u8,
}

impl Default for SpriteSheetAnimation {
    fn default() -> Self {
        Self {
            frames: [0].to_vec(),
            fps: 12,
        }
    }
}

impl SpriteSheetAnimation {
    pub fn from_frames(frames: Vec<usize>, fps: u8) -> Self {
        Self {
            frames,
            fps,
            ..Default::default()
        }
    }

    //    fn from_range(index_range: RangeInclusive<u32>) -> Self {
    //        Self::from_iter(index_range)
    //    }
    //
    //    fn from_iter(indices: impl IntoIterator<Item = u32>) -> Self {
    //        indices.into_iter().map(|index| index).collect()
    //    }

    fn next_frame(&self, frame: usize) -> usize {
        return frame % self.frames.len();
    }
}

#[derive(Component, Debug)]
pub struct SpriteSheetAnimationState {
    current_frame: usize,
    timer: Timer,
}

impl Default for SpriteSheetAnimationState {
    fn default() -> Self {
        SpriteSheetAnimationState {
            current_frame: 0,
            timer: Timer::from_seconds(0.1, true),
        }
    }
}

impl SpriteSheetAnimationState {
    fn new(animation: &SpriteSheetAnimation) -> Self {
        SpriteSheetAnimationState {
            timer: Timer::from_seconds(1.0 / animation.fps as f32, true),
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        time: &Res<Time>,
        mut sprite: impl DerefMut<Target = TextureAtlasSprite>,
        animation: &SpriteSheetAnimation,
    ) {
        self.timer.tick(time.delta());
        if self.timer.finished() {
            sprite.index = animation.next_frame(self.next());
        }
    }

    fn next(&mut self) -> usize {
        self.current_frame += 1;
        self.current_frame
    }
}

pub fn add_animation_state(
    mut commands: Commands,
    animation_defs: Res<Assets<SpriteSheetAnimation>>,
    query: Query<(Entity, &Handle<SpriteSheetAnimation>), (Without<SpriteSheetAnimationState>,)>,
) {
    for (entity, anim_handle) in query.iter() {
        let animation = animation_defs.get(anim_handle).unwrap();
        commands
            .entity(entity)
            .insert(SpriteSheetAnimationState::new(animation));
    }
}

pub fn animate(
    time: Res<Time>,
    animation_defs: Res<Assets<SpriteSheetAnimation>>,
    mut animations: Query<(
        &mut TextureAtlasSprite,
        &Handle<SpriteSheetAnimation>,
        &mut SpriteSheetAnimationState,
    )>,
) {
    for (sprite, animation, mut state) in
        animations
            .iter_mut()
            .filter_map(|(sprite, anim_handle, state)| {
                animation_defs
                    .get(anim_handle)
                    .map(|anim| (sprite, anim, state))
            })
    {
        state.update(&time, sprite, animation);
    }
}

pub fn maintenance_systems() -> SystemSet {
    SystemSet::new().with_system(add_animation_state)
}

pub fn post_update_systems() -> SystemSet {
    SystemSet::new().with_system(animate)
}

#[derive(Default)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SpriteSheetAnimation>()
            .add_system_set_to_stage(CoreStage::PreUpdate, maintenance_systems())
            .add_system_set_to_stage(CoreStage::Update, post_update_systems());
    }
}
