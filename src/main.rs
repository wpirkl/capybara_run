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

mod plugin_signs;
use plugin_signs::SignPlugin;

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
                        title: "Capy Run!".to_string(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(Startup, (setup_camera, setup_background))
        .add_plugins(Model)
        .add_plugins(Scoreboard)
        .add_plugins(GroundPlugin)
        .add_plugins(SignPlugin)
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

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create multiple horizontal bands for gradient effect
    let num_bands = 20;
    let band_height = 800.0 / num_bands as f32;
    
    for i in 0..num_bands {
        let t = i as f32 / (num_bands - 1) as f32;
        
        // Interpolate from light blue (top) to white (bottom)
        let r = 0.7 + (1.0 - 0.7) * t;
        let g = 0.85 + (1.0 - 0.85) * t;
        let b = 1.0;
        
        let color = materials.add(ColorMaterial::from(Color::srgb(r, g, b)));
        let mesh = meshes.add(Rectangle::new(1200.0, band_height));
        
        let y = 400.0 - (i as f32 * band_height) - band_height / 2.0;
        
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(color),
            Transform::from_xyz(0.0, y, -10.0),
        ));
    }
}
