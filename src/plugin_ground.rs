//! Ground plugin module

use bevy::prelude::*;
use rand::Rng;

use crate::constants::*;
use crate::model::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ground)
            .add_systems(Update, move_ground);
    }
}


#[derive(Component, Debug, Clone, Copy, PartialEq)]
enum GroundType {
    Dirt,
    Grass,
    Water,
}

#[derive(Component)]
struct GroundTile;

#[derive(Resource)]
struct GroundTextures {
    dirt: Handle<Image>,
    grass: Handle<Image>,
    water: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn setup_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load ground textures
    let dirt_texture = asset_server.load("textures/ground/dirt_top.png");
    let grass_texture = asset_server.load("textures/ground/grass_top.png");
    let water_texture = asset_server.load("textures/ground/water_top.png");

    // Create texture atlas layout (1 sprite, 240x240)
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    // Store textures as a resource for spawning new tiles
    commands.insert_resource(GroundTextures {
        dirt: dirt_texture.clone(),
        grass: grass_texture.clone(),
        water: water_texture.clone(),
        layout: layout_handle.clone(),
    });

    // Calculate how many tiles we need to fill the screen + 1 extra
    let tiles_needed = (WINDOW_WIDTH / SCALED_TILE_SIZE).ceil() as usize + 1;

    // Spawn initial tiles from left to right
    for i in 0..tiles_needed {
        let x = -WINDOW_WIDTH / 2.0 + (i as f32 * SCALED_TILE_SIZE);
        spawn_ground_tile(&mut commands, x, &dirt_texture, &grass_texture, &water_texture, &layout_handle);
    }
}

fn spawn_ground_tile(
    commands: &mut Commands,
    x: f32,
    dirt_texture: &Handle<Image>,
    grass_texture: &Handle<Image>,
    water_texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
) {
    // Randomly choose a ground type
    let mut rng = rand::rng();
    let ground_type = match rng.random_range(0..3) {
        0 => GroundType::Dirt,
        1 => GroundType::Grass,
        _ => GroundType::Water,
    };

    let texture = match ground_type {
        GroundType::Dirt => dirt_texture.clone(),
        GroundType::Grass => grass_texture.clone(),
        GroundType::Water => water_texture.clone(),
    };

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, GROUND_Y + (WINDOW_HEIGHT * 0.33), 1.0).with_scale(Vec3::splat(TILE_SCALE)),
        GroundTile,
        ground_type,
    ));
}

fn move_ground(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    textures: Res<GroundTextures>,
    mut query: Query<(Entity, &mut Transform), With<GroundTile>>,
) {
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

    // Check if we need to spawn a new tile on the right
    // Spawn when the rightmost tile has moved far enough left to leave a gap
    if rightmost_x < right_edge - SCALED_TILE_SIZE / 2.0 {
        let new_x = rightmost_x + SCALED_TILE_SIZE;
        spawn_ground_tile(
            &mut commands,
            new_x,
            &textures.dirt,
            &textures.grass,
            &textures.water,
            &textures.layout,
        );
    }
}