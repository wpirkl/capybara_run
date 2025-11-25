use bevy::prelude::*;

use crate::model::{GameData, GameStart, GameState, PlayerJump};


pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_keyboard_input)
           .add_systems(Update, handle_input);
    }
}


fn setup_keyboard_input(mut commands: Commands) {


}


fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameData>
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
                commands.trigger(GameStart);
            }
        }
}
}