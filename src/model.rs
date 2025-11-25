use bevy::prelude::*;

use crate::constants::INITIAL_VELOCITY;


#[derive(PartialEq)]
pub enum GameState {
    WaitingForStart,
    Running,
    Dead,
}

#[derive(Resource)]
pub struct GameData {
    pub game_state: GameState,
    pub previous_score: usize,
    pub current_score: usize,
    pub velocity: f32,
}


#[derive(Event)]
pub struct GameEnd;


#[derive(Event)]
pub struct GameStart;


#[derive(Event)]
pub struct PlayerJump;


pub struct Model;
impl Plugin for Model {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameData {
            game_state: GameState::WaitingForStart,
            previous_score: 0,
            current_score: 0,
            velocity: INITIAL_VELOCITY
        });
    }
}

