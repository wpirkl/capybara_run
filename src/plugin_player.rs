//! Player plugin module

use std::time::Duration;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (handle_input, execute_animations, update_jump));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
enum PlayerState {
    Running,
    Jumping,
    Dead,
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct Jump {
    velocity: f32,
    gravity: f32,
    ground_y: f32,
}

#[derive(Component)]
struct PlayerSpritesheets {
    running_texture: Handle<Image>,
    running_layout: Handle<TextureAtlasLayout>,
    jumping_texture: Handle<Image>,
    jumping_layout: Handle<TextureAtlasLayout>,
    dead_texture: Handle<Image>,
    dead_layout: Handle<TextureAtlasLayout>,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load all spritesheets
    let running_texture = asset_server.load("textures/capybara_running.png");
    let jumping_texture = asset_server.load("textures/capybara_jumping.png");
    let dead_texture = asset_server.load("textures/capybara_dead.png");

    // Create layouts for each spritesheet
    // Running: 2 sprites in 1 row, 2 columns (240x240 each)
    let running_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 2, 1, None, None);
    let running_layout_handle = texture_atlas_layouts.add(running_layout);

    // Jumping: 2 sprites in 1 row, 2 columns (240x240 each) - up and down
    let jumping_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 2, 1, None, None);
    let jumping_layout_handle = texture_atlas_layouts.add(jumping_layout);

    // Dead: 1 sprite (240x240)
    let dead_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let dead_layout_handle = texture_atlas_layouts.add(dead_layout);

    // Calculate player position
    let player_x = -600.0 + (1200.0 * 0.20);
    let player_y = -400.0 + (800.0 * 0.33);

    // Running animation config (4 FPS, 2 frames)
    let animation_config = AnimationConfig::new(0, 1, 4);

    // Create the player sprite with running state
    commands.spawn((
        Sprite {
            image: running_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: running_layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(player_x, player_y, 0.0).with_scale(Vec3::splat(0.5)),
        PlayerSprite,
        PlayerState::Running,
        animation_config,
        PlayerSpritesheets {
            running_texture,
            running_layout: running_layout_handle,
            jumping_texture,
            jumping_layout: jumping_layout_handle,
            dead_texture,
            dead_layout: dead_layout_handle,
        },
        Jump {
            velocity: 0.0,
            gravity: -980.0,
            ground_y: player_y,
        },
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerState, &Jump), With<PlayerSprite>>,
) {
    for (mut state, jump) in &mut query {
        // Only allow jumping if on ground and in running state
        if keyboard.just_pressed(KeyCode::Space) && *state == PlayerState::Running && jump.velocity == 0.0 {
            *state = PlayerState::Jumping;
        }
    }
}

fn update_jump(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Jump, &mut PlayerState, &mut Sprite), With<PlayerSprite>>,
) {
    for (mut transform, mut jump, mut state, mut sprite) in &mut query {
        if *state == PlayerState::Jumping {
            // Apply jump velocity on state change
            if jump.velocity == 0.0 {
                jump.velocity = 500.0; // Initial jump velocity
            }

            // Apply gravity
            jump.velocity += jump.gravity * time.delta_secs();
            transform.translation.y += jump.velocity * time.delta_secs();

            // Update sprite index based on velocity (0 = up, 1 = down)
            if let Some(atlas) = &mut sprite.texture_atlas {
                if jump.velocity > 0.0 {
                    atlas.index = 0; // Going up
                } else {
                    atlas.index = 1; // Going down
                }
            }

            // Check if landed
            if transform.translation.y <= jump.ground_y {
                transform.translation.y = jump.ground_y;
                jump.velocity = 0.0;
                *state = PlayerState::Running;
            }
        }
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationConfig,
        &mut Sprite,
        &PlayerState,
        &PlayerSpritesheets,
    )>,
) {
    for (mut config, mut sprite, state, spritesheets) in &mut query {
        // Switch spritesheet based on state
        match state {
            PlayerState::Running => {
                if sprite.image != spritesheets.running_texture {
                    sprite.image = spritesheets.running_texture.clone();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.layout = spritesheets.running_layout.clone();
                        atlas.index = 0;
                    }
                    config.first_sprite_index = 0;
                    config.last_sprite_index = 1;
                    config.fps = 4;
                }

                // Animate running
                config.frame_timer.tick(time.delta());
                if config.frame_timer.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        if atlas.index >= config.last_sprite_index {
                            atlas.index = config.first_sprite_index;
                        } else {
                            atlas.index += 1;
                        }
                    }
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
            PlayerState::Jumping => {
                if sprite.image != spritesheets.jumping_texture {
                    sprite.image = spritesheets.jumping_texture.clone();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.layout = spritesheets.jumping_layout.clone();
                        atlas.index = 0;
                    }
                }
                // Jump animation is handled in update_jump based on velocity
            }
            PlayerState::Dead => {
                if sprite.image != spritesheets.dead_texture {
                    sprite.image = spritesheets.dead_texture.clone();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.layout = spritesheets.dead_layout.clone();
                        atlas.index = 0;
                    }
                }
                // Dead state has no animation
            }
        }
    }
}