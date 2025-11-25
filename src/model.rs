use bevy::prelude::*;

use crate::constants::INITIAL_VELOCITY;


pub enum GameState {
    Starting,
    Running,
    Dead,
}

#[derive(Resource)]
pub struct Game{
    pub game_state: GameState,
    pub previous_score: usize,
    pub current_score: usize,
    pub velocity: f32,
}


pub struct Model;
impl Plugin for Model {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game {
            game_state: GameState::Starting,
            previous_score: 0,
            current_score: 0,
            velocity: INITIAL_VELOCITY
        });
    }
}
