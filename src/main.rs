mod animation;
mod animator;
mod camera;
mod game;
mod input;
mod mouse;

use bevy::{prelude::App, DefaultPlugins};
use game::GooMainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GooMainPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
