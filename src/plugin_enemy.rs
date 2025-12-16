//! Enemy plugin module

use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;

use crate::constants::*;
use crate::model::{GameData, GameReset, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, (sync_enemy_sprites, execute_animations))
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

#[derive(Component, Clone)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}


#[derive(Clone)]
struct EnemyTexture {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    animation: AnimationConfig,
}


#[derive(Resource, Clone)]
struct EnemyTextures {
    eagle: EnemyTexture,
    lion: EnemyTexture,
    croco: EnemyTexture
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

/// Controller component: normalized X where 0.0 == player X, 1.0 == right edge
#[derive(Component)]
struct Enemy {
    normalized_x: f32,
}

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

    let textures = EnemyTextures{
        eagle: EnemyTexture { image: eagle_texture.clone(), layout: eagle_layout_handle.clone(), animation: AnimationConfig::new(0, 0, 1) },
        lion: EnemyTexture { image: lion_texture.clone(), layout: lion_layout_handle.clone(), animation: AnimationConfig::new(0, 0, 1) },
        croco: EnemyTexture { image: croco_texture.clone(), layout: croco_layout_handle.clone(), animation: AnimationConfig::new(0, 0, 1) },
    };

    commands.insert_resource(textures.clone());

    // compute normalized initial x from ENEMY_INITIAL_X
    let right_edge = WINDOW_WIDTH / 2.0;
    let span = right_edge - PLAYER_X;
    let normalized_initial = (ENEMY_INITIAL_X - PLAYER_X) / span;
    spawn_enemy(&mut commands, normalized_initial, &textures);

}


fn spawn_enemy(
    commands: &mut Commands,
    normalized_x: f32,
    textures: &EnemyTextures,
) {
    // Randomly choose an enemy type
    let mut rng = rand::rng();
    let enemy_type = match rng.random_range(0..3) {
        0 => EnemyType::Eagle,
        1 => EnemyType::Lion,
        _ => EnemyType::Croco,
    };
    
    let y = match enemy_type {
        EnemyType::Eagle => ENEMY_FLYING_Y,
        _ => ENEMY_WALKING_Y
    };
    
    let enemy_texture = match enemy_type {
        EnemyType::Eagle => textures.eagle.clone(),
        EnemyType::Lion => textures.lion.clone(),
        EnemyType::Croco => textures.croco.clone(),
    };

    let enemy_distance = rng.random_range(ENEMY_MINIMUM_SPACE..ENEMY_MAXIMUM_SPACE);

    // We'll spawn at normalized_x offset by enemy_distance in world space converted to normalized
    let right_edge = WINDOW_WIDTH / 2.0;
    let span = right_edge - PLAYER_X;
    let offset_norm = enemy_distance / span;
    let spawn_norm = normalized_x + offset_norm;

    // compute world x from normalized
    let world_x = PLAYER_X + spawn_norm * span;

    commands.spawn((
        Sprite{
            image: enemy_texture.image.clone(),
            texture_atlas: Some(TextureAtlas { layout: enemy_texture.layout.clone(), index: 0 }),
            ..default()
        },
        Transform::from_xyz(world_x, y, 0.).with_scale(Vec3::splat(TILE_SCALE)),
        EnemySprite,
        enemy_type,
        enemy_texture.animation.clone(),
        Enemy { normalized_x: spawn_norm },
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


/// Sync sprite transforms from normalized controller
fn sync_enemy_sprites(
    mut query: Query<(&Enemy, &EnemyType, &mut Transform), With<EnemySprite>>,
) {
    let right_edge = WINDOW_WIDTH / 2.0;
    let span = right_edge - PLAYER_X;

    for (enemy, enemy_type, mut transform) in &mut query {
        let world_x = PLAYER_X + enemy.normalized_x * span;
        let y = match enemy_type {
            EnemyType::Eagle => ENEMY_FLYING_Y,
            _ => ENEMY_WALKING_Y,
        };
        transform.translation.x = world_x;
        transform.translation.y = y;
    }
}


fn move_enemy(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameData>,
    textures: Res<EnemyTextures>,
    mut query: Query<(Entity, &mut Enemy), With<EnemySprite>>,
) {
    match game.game_state {
        GameState::Running => {
            let move_distance = game.velocity * time.delta_secs();
            let left_edge_world = -WINDOW_WIDTH / 2.0 - SCALED_TILE_SIZE;
            let right_edge_world = WINDOW_WIDTH / 2.0;
            let span = right_edge_world - PLAYER_X;
            let left_edge_norm = (left_edge_world - PLAYER_X) / span;

            let mut rightmost_norm = f32::MIN;

            // Move enemies by converting move_distance into normalized space
            let move_norm = move_distance / span;

            for (entity, mut enemy) in &mut query {
                enemy.normalized_x -= move_norm;

                if enemy.normalized_x > rightmost_norm {
                    rightmost_norm = enemy.normalized_x;
                }

                if enemy.normalized_x < left_edge_norm {
                    commands.entity(entity).despawn();
                }
            }

            // Spawn when rightmost normalized is sufficiently left
            if rightmost_norm < 1.0 - (SCALED_TILE_SIZE / span) / 2.0 {
                spawn_enemy(&mut commands, rightmost_norm, &textures);
            }
        }
        _ => {}
    }
}


fn handle_enemy_reset(
    _evt: On<GameReset>,
    mut commands: Commands,
    textures: Res<EnemyTextures>,
    enemy_query: Query<(Entity), With<EnemySprite>>,
)
{
    for enemy_entity in & enemy_query {
        commands.entity(enemy_entity).despawn(); 
    }

    // respawn at normalized initial
    let right_edge = WINDOW_WIDTH / 2.0;
    let span = right_edge - PLAYER_X;
    let normalized_initial = (ENEMY_INITIAL_X - PLAYER_X) / span;
    spawn_enemy(&mut commands, normalized_initial, &textures);
}
