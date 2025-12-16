use std::time::Duration;
use bevy::prelude::*;

use crate::constants::*;
use crate::model::{GameData, GameEnd, GameReset, GameState, PlayerJump};

pub struct TombstonePlugin;

impl Plugin for TombstonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_tombstone)
            .add_observer(handle_tombstone_reset);
    }
}

#[derive(Component)]
struct TombstoneSprite;


#[derive(Component)]
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

    commands.spawn((
        TombstoneSprite,
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                    layout: layout_handle.clone(),
                    index: 0,
                }),
            ..default()
        },
        Transform::from_xyz(PLAYER_X + 100., PLAYER_GROUND, 0.).with_scale(Vec3::splat(TILE_SCALE)),
        TombstoneSpritesheets{
            texture: texture_handle,
            layout: layout_handle,
        }
    ));
}

fn move_tombstone(
    game: Res<GameData>,
    mut query: Query<(&mut Transform, &mut Visibility), With<TombstoneSprite>>
)
{

}


fn handle_tombstone_reset(
    _evt: On<GameReset>,
    mut query: Query<&mut Visibility, With<TombstoneSprite>>
)
{

}
