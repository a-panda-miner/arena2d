use ar_core::{AppState, MapSet};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_fast_tilemap::{bundle::MapBundleManaged, map::Map};
use rand::prelude::*;

const TILE_SIZE: u32 = 16;

// Playable area
const ARENA_RADIUS: u32 = 1000;

// Used for generation of the map and the spawn of enemies
const ARENA_WIDTH: u32 = (ARENA_RADIUS * 2) + TILE_SIZE;
const ARENA_HEIGHT: u32 = (ARENA_RADIUS * 2) + TILE_SIZE;

#[derive(AssetCollection, Resource)]
pub struct TilesetHandle {
    #[asset(path = "map/tileset.png")]
    pub sprite: Handle<Image>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InBattle),
            map_builder_tileset.in_set(MapSet),
        );
    }
}

/// Builds the visual tileset of the game
fn map_builder_tileset(
    mut commands: Commands,
    tile_set: Res<TilesetHandle>,
    mut materials: ResMut<Assets<Map>>,
) {
    let mut rng = thread_rng();

    let map = Map::builder(
        // The size of the map
        uvec2(ARENA_WIDTH * 4, ARENA_HEIGHT * 4),
        // Tile atlas
        tile_set.sprite.clone(),
        // Tile size
        vec2(16., 16.),
    )
    // Set the index of each tile
    .build_and_set(|_| rng.gen_range(1..4));

    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));
}
