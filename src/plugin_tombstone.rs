use std::time::Duration;
use bevy::prelude::*;

use crate::constants::*;
use crate::model::{GameData, GameReset, GameState};

pub struct TombstonePlugin;

impl Plugin for TombstonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_tombstone)
            .add_observer(handle_tombstone_reset);
    }
}

#[derive(Component)]
struct TombstoneSprite;


#[derive(Resource, Clone)]
struct TombstoneSpritesheets { 
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
)
{
    let texture_handle = asset_server.load("textures/ground/rip.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    // Store the spritesheets resource for spawning later
    commands.insert_resource(TombstoneSpritesheets{
        texture: texture_handle,
        layout: layout_handle,
    });
}

fn move_tombstone(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameData>,
    mut query: Query<(Entity, &mut Transform), With<TombstoneSprite>>
)
{
    if game.game_state == GameState::Running {
        for (entity, mut transform) in &mut query {
            
            let move_distance = game.velocity * time.delta_secs();

            transform.translation.x -= move_distance;
            if transform.translation.x < -WINDOW_WIDTH / 2.0 - SCALED_TILE_SIZE {
                commands.entity(entity).despawn();
            }
        }
    }
}


fn handle_tombstone_reset(
    _evt: On<GameReset>,
    mut commands: Commands,
    game: Res<GameData>,
    spritesheets: Res<TombstoneSpritesheets>,
    query: Query<Entity, With<TombstoneSprite>>
)
{
    // Despawn any existing tombstones
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // Spawn new tombstone if there is a previous score
    if let Some(previous_score) = game.previous_score {

        let spawn_x = previous_score + PLAYER_X;
        commands.spawn((
            TombstoneSprite,
            Sprite {
                image: spritesheets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: spritesheets.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(spawn_x, PLAYER_GROUND, 0.).with_scale(Vec3::splat(TILE_SCALE)),
        ));
    
    }
}
