use bevy::prelude::*;
use bevy::math::{uvec2, vec2};
use bevy_asset_loader::prelude::*;
use bevy_fast_tilemap::{Map, MapBundleManaged};
use ar_core::{AppState, MapSet};
use rand::prelude::*;

const TILE_SIZE: u32 = 16;
const CHUNK_SIZE: UVec2 = UVec2 { x: 10, y: 10};

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


enum TileType {
    Grass,
    Rock,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InBattle), map_builder.in_set(MapSet));
    }
}

fn map_builder(mut commands: Commands,tile_set: Res<TilesetHandle>, mut materials: ResMut<Assets<Map>>) {
    let mut rng = rand::thread_rng();

    let map = Map::builder(
        // The size of the map
        uvec2(ARENA_WIDTH/8, ARENA_HEIGHT/8),
        // Tile atlas
        tile_set.sprite.clone(),
        // Tile size
        vec2(16., 16.),
    )
    // Set the index of each tile
    .build_and_set(|_| rng.gen_range(1..4));

    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));
}