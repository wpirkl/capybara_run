use bevy::prelude::*;

use crate::constants::INITIAL_VELOCITY;

#[derive(Resource, Deref, DerefMut)]
pub struct Velocity(pub f32);


#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);


#[derive(Resource, Deref, DerefMut)]
pub struct PreviousScore(pub usize);


pub struct Model;
impl Plugin for Model {
    fn build(&self, app: &mut App) {
        app.insert_resource(Velocity(INITIAL_VELOCITY))
           .insert_resource(Score(0))
           .insert_resource(PreviousScore(0));
    }
}
