//! Main application file

use bevy::prelude::*;
use bevy::window::WindowResolution;

mod plugin_player;
use plugin_player::PlayerPlugin;

mod plugin_enemy;
use plugin_enemy::EnemyPlugin;

mod plugin_ground;
use plugin_ground::GroundPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1200, 800),
                        resizable: false,
                        title: "Bevy Player".to_string(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GroundPlugin)
        .run();
}

// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
