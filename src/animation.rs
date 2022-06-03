use bevy::core::{Time, Timer};
use bevy::ecs::component::Component;
use bevy::prelude::Res;
use bevy::reflect::TypeUuid;
use bevy::sprite::TextureAtlasSprite;
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

#[derive(Component)]
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
    fn new(animation: SpriteSheetAnimation) -> Self {
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
