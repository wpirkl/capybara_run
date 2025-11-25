//! Enemy plugin module

use std::time::Duration;
use bevy::prelude::*;

use crate::constants::*;
use crate::model::{GameData, GameReset, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, execute_animations)
            .add_systems(FixedUpdate, move_enemy)
            .add_observer(handle_enemy_reset);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
enum EnemyType {
    Eagle,
    Lion,
    Croco,
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
pub struct EnemySprite;

fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load enemy spritesheets
    let eagle_texture = asset_server.load("textures/enemies/eagle/flying.png");
    let lion_texture = asset_server.load("textures/enemies/lion/running.png");
    let croco_texture = asset_server.load("textures/enemies/crocodile/running.png");

    // Create layouts for each enemy type
    // Eagle: 1 sprite (1 row, 1 column) - assume 240x240 like player
    let eagle_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let eagle_layout_handle = texture_atlas_layouts.add(eagle_layout);

    // Lion: 1 rows, 1 column - each row is a different enemy (240x240 each)
    let lion_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let lion_layout_handle = texture_atlas_layouts.add(lion_layout);

        // Croco: 1 rows, 1 column - each row is a different enemy (240x240 each)
    let croco_layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let croco_layout_handle = texture_atlas_layouts.add(croco_layout);

    // Calculate enemy positions at right side of window
    // Window size: 1200 x 800
    // Right side position, 33% from bottom
    let enemy_x = 600.0 - 100.0; // Near right edge (leaving some margin)
    let enemy_y = PLAYER_GROUND;

    let animation_config = AnimationConfig::new(0, 0, 1);

    // Spawn flying enemy
    commands.spawn((
        Sprite {
            image: eagle_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: eagle_layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(enemy_x, enemy_y + 120.0, 0.0).with_scale(Vec3::splat(TILE_SCALE)),
        EnemySprite,
        EnemyType::Eagle,
        animation_config,
    ));

    let animation_config = AnimationConfig::new(0, 0, 1);
    
    // Spawn walking enemy 1 (first row, index 0)
    commands.spawn((
        Sprite {
            image: lion_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: lion_layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(enemy_x, enemy_y + 0.0, 0.0).with_scale(Vec3::splat(TILE_SCALE)),
        EnemySprite,
        EnemyType::Lion,
        animation_config,
    ));

    let animation_config = AnimationConfig::new(0, 0, 1);

    // Spawn walking enemy 2 (second row, index 1)
    commands.spawn((
        Sprite {
            image: croco_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: croco_layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(enemy_x - 120., enemy_y, 0.0).with_scale(Vec3::splat(TILE_SCALE)),
        EnemySprite,
        EnemyType::Croco,
        animation_config,
    ));
}

// This system loops through all the sprites in the `TextureAtlas`
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<EnemySprite>>,
) {
    for (mut config, mut sprite) in &mut query {

        // Animate enemy
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
}

fn move_enemy(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameData>,
    mut query: Query<(Entity, &mut Transform), With<EnemySprite>>,
) {
    match game.game_state {
        GameState::Running => {
            let move_distance = game.velocity * time.delta_secs();
            let left_edge = -WINDOW_WIDTH / 2.0 - SCALED_TILE_SIZE;
            let right_edge = WINDOW_WIDTH / 2.0;

            let mut rightmost_x = f32::MIN;

            for (entity, mut transform) in &mut query {
                // Move tile to the left
                transform.translation.x -= move_distance;

                // Track the rightmost tile position
                if transform.translation.x > rightmost_x {
                    rightmost_x = transform.translation.x;
                }

                // If tile has moved off the left edge, despawn it
                if transform.translation.x < left_edge {
                    commands.entity(entity).despawn();
                }
            }
        }
        _ => {}
    }
}


fn handle_enemy_reset(
    _evt: On<GameReset>,
    mut commands: Commands,
    enemy_query: Query<(Entity), With<EnemySprite>>,
)
{
    for enemy_entity in & enemy_query {

        commands.entity(enemy_entity).despawn();
    }

}
