use bevy::prelude::*;

use crate::model::{GameData, GameEnd, GameStart, GameState};

pub struct GameController;

impl Plugin for GameController {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_distance)
           .add_observer(handle_game_start)
           .add_observer(handle_game_end);
    }
}


fn handle_game_start(
    _evt: On<GameStart>,
    mut game: ResMut<GameData>
)
{
    game.game_state = GameState::Running;
}


fn handle_game_end(
    _evt: On<GameEnd>,
    mut game: ResMut<GameData>
)
{
    game.game_state = GameState::Dead;
}


fn update_distance(
    time: Res<Time>,
    mut game: ResMut<GameData>
)
{
    match game.game_state {
        GameState::Running => {
            let move_distance = game.velocity * time.delta_secs();
            game.current_score += move_distance as usize;
        }
        _ => {}
    }
}
