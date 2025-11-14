//! Sprite animation module for running animations

use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct RunningAnimation {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: f32,
    frame_timer: Timer,
}

impl RunningAnimation {
    pub fn new(first: usize, last: usize, fps: f32) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Timer::default(),
        }
    }
}

fn timer_from_fps(fps: f32) -> Timer {
    Timer::new(Duration::from_secs_f32(1.0 / fps), TimerMode::Once)
}

pub fn animate_running_sprites(
    time: Res<Time>,
    mut query: Query<(&mut RunningAnimation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in &mut query {

        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= animation.last_sprite_index {
                    atlas.index = animation.first_sprite_index;
                } else {
                    atlas.index += 1;
                }
            }
            animation.frame_timer = timer_from_fps(animation.fps);
        }
    }
}