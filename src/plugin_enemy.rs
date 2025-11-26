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

    spawn_enemy(&mut commands, ENEMY_INITIAL_X, &textures);

}


fn spawn_enemy(
    commands: &mut Commands,
    x: f32,
    textures: &EnemyTextures
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

    commands.spawn((
        Sprite{
            image: enemy_texture.image.clone(),
            texture_atlas: Some(TextureAtlas { layout: enemy_texture.layout.clone(), index: 0 }),
            ..default()
        },
        Transform::from_xyz(x + enemy_distance, y, 0.).with_scale(Vec3::splat(TILE_SCALE)),
        EnemySprite,
        enemy_type,
        enemy_texture.animation.clone()
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
    textures: Res<EnemyTextures>,
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

                // Track the rightmost enemy position
                if transform.translation.x > rightmost_x {
                    rightmost_x = transform.translation.x;
                }

                // If tile has moved off the left edge, despawn it
                if transform.translation.x < left_edge {
                    commands.entity(entity).despawn();
                }
            }

            // Check if we need to spawn a new tile on the right
            // Spawn when the rightmost tile has moved far enough left to leave a gap
            if rightmost_x < right_edge - SCALED_TILE_SIZE / 2. {
                
                spawn_enemy(&mut commands, rightmost_x, &textures);
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

    spawn_enemy(&mut commands, ENEMY_INITIAL_X, &textures);
}
