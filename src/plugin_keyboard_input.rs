use bevy::prelude::*;

use crate::model::{GameData, GameStart, GameState, PlayerJump, GameReset};


pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut App) {
        app//.add_systems(Startup, setup_keyboard_input)
           .add_systems(FixedUpdate, handle_input);
    }
}



fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    game: ResMut<GameData>
) {
    if keyboard.just_pressed(KeyCode::Space){
        match game.game_state {
            GameState::WaitingForStart => {
                commands.trigger(GameStart);
            }

            GameState::Running => {
                commands.trigger(PlayerJump);
            }

            GameState::Dead => {
                commands.trigger(GameReset);
            }

            _ => {}
        }
}
}