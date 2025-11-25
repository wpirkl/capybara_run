use bevy::prelude::*;

use crate::constants::*;
use crate::model::*;

pub struct SignPlugin;

impl Plugin for SignPlugin {
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, setup_sign)
           .add_systems(FixedUpdate, move_sign)
           .add_observer(handle_sign_reset);
    }
}


#[derive(Component)]
struct SignSprite;


#[derive(Resource)]
struct SignTextures {
    sign: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn setup_sign(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let sign_texture = asset_server.load("textures/ground/distance_sign.png");

    // Create texture atlas layout (1 sprite, 240x240)
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(240), 1, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(SignTextures {
        sign: sign_texture.clone(),
        layout: layout_handle.clone()
    });

    spawn_sign(&mut commands, PLAYER_X + 1000., &sign_texture, &layout_handle, 1000.);
}


fn spawn_sign(
    commands: &mut Commands,
    x: f32,
    sign_texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    sign_distance: f32
) {
    let texture = sign_texture.clone();
    let distance = (sign_distance / 1000.).round() as usize;

    let sign_entity = commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas { layout: layout.clone(), index: 0 }), ..default()
        },
        Transform::from_xyz(x, PLAYER_GROUND, -1.0).with_scale(Vec3::splat((TILE_SCALE))),
        SignSprite
    )).id();

    // Spawn the text as a child of the sign
    let text_entity = commands.spawn((
        Text2d::new(distance.to_string() + "k"),
        TextFont {
            font_size: SIGN_FONT_SIZE,
            ..default()
        },
        TextColor(SIGN_COLOR),
        Transform::from_xyz(0.0, SIGN_OFFSET_Y, 1.0), // Position relative to parent
    )).id();

    // Set up parent-child relationship
    commands.entity(sign_entity).add_child(text_entity);
}


fn move_sign(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameData>,
    textures: Res<SignTextures>,
    mut query: Query<(Entity, &mut Transform), With<SignSprite>>,
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

            if rightmost_x < right_edge - SCALED_TILE_SIZE / 2. {
                
                let new_x = rightmost_x + 1000.;
                let distance = new_x - PLAYER_X + game.current_score;
                spawn_sign(&mut commands, new_x, &textures.sign, &textures.layout, distance);
            }

        }
        _ => {}
    }
}


fn handle_sign_reset(
    _evt: On<GameReset>,
    mut commands: Commands,
    game: Res<GameData>,
    textures: Res<SignTextures>,
    sign_query: Query<(Entity), With<SignSprite>>,
)
{
    for enemy_entity in & sign_query {

        commands.entity(enemy_entity).despawn();
    }

    spawn_sign(&mut commands, PLAYER_X + 1000., &textures.sign, &textures.layout, 1000.);
}
