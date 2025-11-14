//! Enemy plugin module with dynamic enemy factory

use bevy::prelude::*;
use std::path::PathBuf;

use crate::sprite_animation_running::{RunningAnimation, animate_running_sprites};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, animate_running_sprites);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Flying,
    Running,
}

#[derive(Component)]
pub struct EnemySprite {
    pub enemy_name: String,
}

#[derive(Debug, Clone)]
struct EnemyDefinition {
    name: String,
    enemy_type: EnemyType,
    texture_path: String,
}

struct EnemyFactory {
    enemies: Vec<EnemyDefinition>,
}

impl EnemyFactory {
    fn new() -> Self {
        Self {
            enemies: Vec::new(),
        }
    }

    fn discover_enemies(&mut self) {
        // Define the enemy base path
        let enemy_folders = vec!["eagle", "lion", "crocodile"];
        
        for folder in enemy_folders {
            // Check for flying.png
            let flying_path = format!("textures/enemies/{}/flying.png", folder);
            if Self::texture_exists(&flying_path) {
                self.enemies.push(EnemyDefinition {
                    name: folder.to_string(),
                    enemy_type: EnemyType::Flying,
                    texture_path: flying_path,
                });
            }
            
            // Check for running.png
            let running_path = format!("textures/enemies/{}/running.png", folder);
            if Self::texture_exists(&running_path) {
                self.enemies.push(EnemyDefinition {
                    name: folder.to_string(),
                    enemy_type: EnemyType::Running,
                    texture_path: running_path,
                });
            }
        }
    }

    fn texture_exists(_path: &str) -> bool {
        // In a real implementation, this would check if the file exists
        // For now, we'll assume all textures exist
        // You could use std::fs::metadata or bevy's asset server
        true
    }

    fn spawn_enemy(
        &self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        enemy_def: &EnemyDefinition,
        position: Vec3,
        scale: f32,
    ) {
        // Load the texture
        let texture = asset_server.load(&enemy_def.texture_path);
        
        // Get image dimensions to calculate sprite count
        // Note: In production, you'd want to load this info from a metadata file
        // or calculate it after the image is loaded. For now, we'll default to 1 sprite.
        let sprite_count = Self::get_sprite_count(&enemy_def.texture_path);
        
        // Create texture atlas layout (1 row, N columns)
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(240),
            sprite_count,
            1,
            None,
            None
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let last_index = if sprite_count > 1 { sprite_count - 1 } else { 0 };
        
        // Spawn the enemy
        let mut entity = commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            EnemySprite {
                enemy_name: enemy_def.name.clone(),
            },
            enemy_def.enemy_type,
        ));

        // Add animation component for running enemies with multiple sprites
        if enemy_def.enemy_type == EnemyType::Running && sprite_count > 1 {
            entity.insert(RunningAnimation::new(0, last_index as usize, 4.));
        }
    }

    fn get_sprite_count(_texture_path: &str) -> u32 {
        // In a real implementation, this would:
        // 1. Load the image
        // 2. Get its width
        // 3. Divide by 240 to get the number of sprites
        // For now, return 1 as default
        // You could use image crate or bevy's image loading
        1
    }
}

fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut factory = EnemyFactory::new();
    factory.discover_enemies();

    // Calculate base position (right side, 33% from bottom)
    let enemy_x = 600.0 - 100.0;
    let enemy_y = -400.0 + (800.0 * 0.33);

    // Spawn all discovered enemies
    let mut y_offset = 120.0;
    for enemy_def in &factory.enemies {
        factory.spawn_enemy(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            enemy_def,
            Vec3::new(enemy_x, enemy_y + y_offset, 0.0),
            0.5,
        );
        y_offset -= 120.0; // Stack enemies vertically
    }
}