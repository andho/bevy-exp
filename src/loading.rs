use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};
use std::{fmt::Debug, hash::Hash, iter::repeat};

#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "character/character-sheet.png")]
    player_spritesheet: Handle<Image>,
    #[asset(path = "background/cracked-dirt.png")]
    background_texture: Handle<Image>,
}

pub trait LoadingState1: Debug + Clone + Copy + PartialEq + Eq + Hash + Sync + Send {}

#[derive(Default)]
pub struct LoadingPlugin<T: LoadingState1> {
    loading_state: T,
    next_state: T,
}

impl<T: 'static + LoadingState1> LoadingPlugin<T> {
    pub fn new(loading_state: T, next_state: T) -> Self {
        Self {
            loading_state,
            next_state,
        }
    }
}

impl<T: 'static + LoadingState1> Plugin for LoadingPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProgressTimer(Timer::from_seconds(0.5, true)))
            .add_startup_system(loading_screen_setup)
            .add_loading_state(
                LoadingState::new(self.loading_state)
                    .continue_to_state(self.next_state)
                    .with_collection::<GameAssets>(),
            )
            .add_plugin(ProgressPlugin::new(self.loading_state))
            .add_system(print_progress.run_in_state(self.loading_state));
    }
}

struct ProgressTimer(Timer);

#[derive(Default)]
struct ProgressState {
    tick: usize,
}

fn print_progress(
    time: Res<Time>,
    mut timer: ResMut<ProgressTimer>,
    progress: Option<Res<ProgressCounter>>,
    mut query: Query<&mut Text>,
    mut state: Local<ProgressState>,
) {
    if let Some(progress) = progress {
        if timer.0.tick(time.delta()).just_finished() {
            let length = state.tick % 3;
            let dots = repeat(".").take(length + 1).collect::<String>();
            let mut text = query.single_mut();
            text.sections[1].value = dots;

            state.tick += 1;
        }
    }
}

fn loading_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Loading".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/Ubuntu.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "...".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/Ubuntu.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        });
}
