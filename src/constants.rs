use bevy::{color::Color, ui::Val};

pub const TILE_SIZE: f32 = 240.0;
pub const TILE_SCALE: f32 = 0.5;
pub const SCALED_TILE_SIZE: f32 = TILE_SIZE * TILE_SCALE;
pub const GROUND_Y: f32 = -400.0; // Bottom of the screen
pub const WINDOW_WIDTH: f32 = 1200.;
pub const WINDOW_HEIGHT: f32 = 800.;
pub const INITIAL_VELOCITY: f32 = 200.;
pub const COLLISION_RADIUS: f32 = 60.;

pub const SCOREBOARD_FONT_SIZE: f32 = 33.;
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.);
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
