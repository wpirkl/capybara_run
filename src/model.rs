use bevy::prelude::*;

use crate::constants::INITIAL_VELOCITY;


#[derive(PartialEq)]
pub enum GameState {
    WaitingForStart,
    Running,
    Dead,
    Reset
}


#[derive(Resource)]
pub struct GameData {
    pub game_state: GameState,
    pub previous_score: f32,
    pub current_score: f32,
    pub velocity: f32,
}


#[derive(Event)]
pub struct GameEnd;


#[derive(Event)]
pub struct GameStart;


#[derive(Event)]
pub struct GameReset;


#[derive(Event)]
pub struct PlayerJump;


pub struct Model;
impl Plugin for Model {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameData {
            game_state: GameState::WaitingForStart,
            previous_score: 0.,
            current_score: 0.,
            velocity: INITIAL_VELOCITY
        }).add_observer(handle_model_reset);
    }
}

fn handle_model_reset(
    _evt: On<GameReset>,
    mut game: ResMut<GameData>
)
{
    game.previous_score = game.current_score;
    game.current_score = 0.;
    game.velocity = INITIAL_VELOCITY;
    game.game_state = GameState::WaitingForStart;
}
