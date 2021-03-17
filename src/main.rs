use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MyGameNameHerePlugin)
        .run();
}

const SPEED: f32 = 100.0;
const ANIMATION_FPS: u8 = 12;

pub struct MyGameNameHerePlugin;

impl Plugin for MyGameNameHerePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerActionState { ..Default::default() })
            .add_startup_system(setup.system())
            .add_system(animate.system())
            .add_system(change_animation.system())
            .add_system(player_controller.system())
            .add_system(player_movement.system());
    }
}

struct Player;

struct CharacterMovement {
    prev_velocity: Vec2,
}

impl Default for CharacterMovement {
    fn default() -> Self {
        Self {
            prev_velocity: Vec2::new(0.0, 0.0)
        }
    }
}

struct PlayerActionState {
    velocity: Vec2,
}

impl Default for PlayerActionState {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0.0, 0.0),
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("character/character-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 11, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .with(Animation::new((0, 0), ANIMATION_FPS))
        .with(Player)
    ;
}

enum AnimationState { // not sure if this is needed anymore
    Paused,
    Playing,
}

struct Animation {
    index: (u32, u32),
    current_frame: u32,
    state: AnimationState, // not sure if this is needed anymore
    timer: Timer
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            index: (0, 1),
            current_frame: 0,
            state: AnimationState::Playing,
            timer: Timer::from_seconds(0.1, true),
        }
    }
}

impl Animation {
    fn new(index: (u32, u32), fps: u8) -> Self {
        Animation {
            index,
            timer: Timer::from_seconds(1.0 / fps as f32, true),
            ..Default::default()
        }
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn next(&mut self) -> u32 {
        let diff = self.index.1 - self.index.0 + 1;
        self.current_frame = (self.current_frame + 1) % diff;
        self.current_frame + self.index.0
    }
}

/**
 * TODO right now the first frame of the animation when it's first set it only
 * after the first tick (from timer). This is apparent when FPS is very slow,
 * for example setting fps as 1 on the Idle animation.
 * Need to somehow set the first frame immediately.
 */
fn animate(
    time: Res<Time>,
    mut query: Query<
        (&mut TextureAtlasSprite, &mut Animation)>,
) {
    for (mut sprite, mut anim) in query.iter_mut() {
        let timer = anim.timer_mut();
        timer.tick(time.delta_seconds());
        if timer.finished() {
            sprite.index = anim.next();
        }
    }
}

fn change_animation(
    commands: &mut Commands,
    player_state: Res<PlayerActionState>,
    mut local: Local<CharacterMovement>, // should probably move velocity to a component.
    mut query: Query<Entity, With<Player>>,
) {
    for entity in query.iter_mut() {
        let velocity = player_state.velocity;
        if velocity.length() != 0.0 && local.prev_velocity.length() == 0.0 {
            commands.remove_one::<Animation>(entity);
            commands.insert_one(entity, Animation::new((1, 10), ANIMATION_FPS));
        } else if velocity.length() == 0.0 && local.prev_velocity.length() != 0.0 {
            commands.remove_one::<Animation>(entity);
            commands.insert_one(entity, Animation::new((0, 0), ANIMATION_FPS));
        }
        local.prev_velocity = velocity;
    }
}

fn player_controller(
    mut player_state: ResMut<PlayerActionState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        player_state.velocity.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::S) {
        player_state.velocity.y = -1.0;
    } else {
        player_state.velocity.y = 0.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        player_state.velocity.x = 1.0;
    } else if keyboard_input.pressed(KeyCode::A) {
        player_state.velocity.x = -1.0;
    } else {
        player_state.velocity.x = 0.0;
    }
}

fn player_movement(
    player_state: Res<PlayerActionState>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (_, mut transform) in query.iter_mut() {
        let final_velocity = player_state.velocity.extend(0.0) * SPEED * delta;
        transform.translation += final_velocity;
    }
}
