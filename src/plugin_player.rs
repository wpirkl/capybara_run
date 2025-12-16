//! Player plugin module

use std::time::Duration;
use bevy::prelude::*;

use crate::constants::*;
use crate::model::{GameData, GameEnd, GameReset, GameState, PlayerJump};
use crate::plugin_enemy::EnemySprite;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, execute_animations)
           .add_systems(FixedUpdate, (update_player_physics, check_for_collisions))
           .add_observer(handle_input)
           .add_observer(handle_player_reset);
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

/// Physics component handling jump and collision state
/// y_position is normalized: 0.0 = ground, 1.0 = highest point of jump
#[derive(Component)]
struct Player {
    /// Horizontal position in world space
    position_x: f32,
    /// Normalized vertical position (0.0 = ground, 1.0 = max height)
    y_position: f32,
    /// Current vertical velocity in normalized space per second
    velocity: f32,
    /// Gravity acceleration in normalized space per second^2
    gravity: f32
}

impl Player {
    fn new(position_x: f32) -> Self {
        Self {
            position_x,
            y_position: 0.0,
            velocity: 0.0,
            gravity: -2.0, // normalized gravity
        }
    }

    fn start_jump(&mut self) {
        self.velocity = 2.0; // normalized initial velocity
    }

    fn update(&mut self, delta_time: f32) {
        self.velocity += self.gravity * delta_time;
        self.y_position += self.velocity * delta_time;

        // Land on ground
        if self.y_position <= 0.0 {
            self.y_position = 0.0;
            self.velocity = 0.0;
        }
    }

    /// Returns true if player is in jumping state (off ground)
    fn is_jumping(&self) -> bool {
        self.y_position > 0.0
    }

    /// Convert normalized y position to world space
    fn world_y(&self, ground_y: f32, jump_height: f32) -> f32 {
        ground_y + self.y_position * jump_height
    }
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

const JUMP_HEIGHT: f32 = 200.0; // World space jump height

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load all spritesheets
    let running_texture = asset_server.load("textures/player/running.png");
    let jumping_texture = asset_server.load("textures/player/jumping.png");
    let dead_texture = asset_server.load("textures/player/dead.png");

    // Create layouts for each spritesheet
    let running_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 2, 1, None, None);
    let running_layout_handle = texture_atlas_layouts.add(running_layout);

    let jumping_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 2, 1, None, None);
    let jumping_layout_handle = texture_atlas_layouts.add(jumping_layout);

    let dead_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let dead_layout_handle = texture_atlas_layouts.add(dead_layout);

    let player_x = PLAYER_X;
    let player_ground_y = PLAYER_GROUND;
    let animation_config = AnimationConfig::new(0, 1, 4);

    commands.spawn((
        Sprite {
            image: running_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: running_layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(player_x, player_ground_y, 0.0).with_scale(Vec3::splat(TILE_SCALE)),
        PlayerSprite,
        PlayerState::Running,
        animation_config,
        Player::new(player_x),
        PlayerSpritesheets {
            running_texture,
            running_layout: running_layout_handle,
            jumping_texture,
            jumping_layout: jumping_layout_handle,
            dead_texture,
            dead_layout: dead_layout_handle,
        },
    ));
}

fn handle_input(
    _jump: On<PlayerJump>,
    mut query: Query<(&mut Player, &mut PlayerState), With<PlayerSprite>>,
) {
    for (mut player, mut state) in &mut query {
        // Only allow jumping if on ground and in running state
        if *state == PlayerState::Running && !player.is_jumping() {
            player.start_jump();
            *state = PlayerState::Jumping;
        }
    }
}

fn update_player_physics(
    time: Res<Time>,
    game: Res<GameData>,
    mut query: Query<(&mut Player, &mut PlayerState, &mut Sprite), With<PlayerSprite>>,
) {
    for (mut player, mut state, mut sprite) in &mut query {
        // If physics already indicate the player is off-ground, ensure state reflects that
        if player.is_jumping() && *state == PlayerState::Running {
            *state = PlayerState::Jumping;
        }

        // Update physics only when jumping
        if *state == PlayerState::Jumping || player.is_jumping() {
            player.update(time.delta_secs());

            // Update sprite index based on velocity (0 = up, 1 = down)
            if let Some(atlas) = &mut sprite.texture_atlas {
                if player.velocity > 0.0 {
                    atlas.index = 0; // Going up
                } else {
                    atlas.index = 1; // Going down
                }
            }

            // Check if landed
            if !player.is_jumping() {
                match game.game_state {
                    GameState::Dead => {
                        *state = PlayerState::Dead;
                    }
                    _ => {
                        *state = PlayerState::Running;
                    }
                }
            }
        }
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationConfig,
        &mut Sprite,
        &mut Transform,
        &PlayerState,
        &Player,
        &PlayerSpritesheets,
    )>,
) {
    for (mut config, mut sprite, mut transform, state, player, spritesheets) in &mut query {
        // Update transform based on player physics
        let ground_y = PLAYER_GROUND;
        transform.translation.y = player.world_y(ground_y, JUMP_HEIGHT);

        match *state {
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
            }
            PlayerState::Dead => {
                if sprite.image != spritesheets.dead_texture {
                    sprite.image = spritesheets.dead_texture.clone();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.layout = spritesheets.dead_layout.clone();
                        atlas.index = 0;
                    }
                }
            }
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<(&Player, &mut PlayerState), With<PlayerSprite>>,
    enemy_query: Query<&Transform, With<EnemySprite>>,
) {
    for enemy_transform in &enemy_query {
        for (player, mut player_state) in &mut player_query {
            let player_world_y = player.world_y(PLAYER_GROUND, JUMP_HEIGHT);
            let player_pos = Vec3::new(player.position_x, player_world_y, 0.0);
            let distance = player_pos.distance(enemy_transform.translation);
            if distance < COLLISION_RADIUS {
                if *player_state == PlayerState::Running {
                    *player_state = PlayerState::Dead;
                }
                // If you need to include player id in the GameEnd event, extend that event
                commands.trigger(GameEnd);
            }
        }
    }
}

fn handle_player_reset(
    _evt: On<GameReset>,
    mut player_query: Query<(&mut Player, &mut PlayerState), With<PlayerSprite>>,
) {
    for (mut player, mut player_state) in &mut player_query {
        player.y_position = 0.0;
        player.velocity = 0.0;
        *player_state = PlayerState::Running;
    }
}
