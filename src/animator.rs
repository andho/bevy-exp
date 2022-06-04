use std::{fmt::Display, hash::Hash};

use bevy::{
    prelude::{Commands, Component, Entity, Handle, Query},
    utils::HashMap,
};

use crate::animation::SpriteSheetAnimation;

pub trait AnimationKey: Eq + Hash + Sync + Send + Default + Display {}

#[derive(Component)]
pub struct Animator<T: AnimationKey, U> {
    animations: HashMap<T, Handle<SpriteSheetAnimation>>,
    selector: fn(U) -> T,
}

impl<T: AnimationKey, U> Animator<T, U> {
    pub fn new(animations: HashMap<T, Handle<SpriteSheetAnimation>>, selector: fn(U) -> T) -> Self {
        Self {
            animations,
            selector,
        }
    }

    pub fn select(&self, data: U) -> Handle<SpriteSheetAnimation> {
        let animation: T = (self.selector)(data);

        self.animations.get(&animation).unwrap().clone_weak()
    }
}

pub fn animation_selection<T: AnimationKey + 'static, U: 'static + Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, &Animator<T, U>, &U)>,
) {
    for (entity, animator, anim_data) in query.iter() {
        let animation = animator.select(anim_data.clone());
        commands.entity(entity).insert(animation);
    }
}
