//! Main application file

use bevy::prelude::*;
use bevy::window::WindowResolution;

mod constants;
use constants::*;

mod model;
use model::Model;

mod plugin_scoreboard;
use plugin_scoreboard::Scoreboard;

mod plugin_player;
use plugin_player::PlayerPlugin;

mod plugin_enemy;
use plugin_enemy::EnemyPlugin;

mod plugin_ground;
use plugin_ground::GroundPlugin;

mod plugin_keyboard_input;
use plugin_keyboard_input::KeyboardInputPlugin;

mod plugin_game_controller;
use plugin_game_controller::GameController;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                        resizable: false,
                        title: "Bevy Player".to_string(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup_camera)
        .add_plugins(Model)
        .add_plugins(Scoreboard)
        .add_plugins(GroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(KeyboardInputPlugin)
        .add_plugins(GameController)
        .run();
}


// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
