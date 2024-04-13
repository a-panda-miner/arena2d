use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;


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

struct MapPlugin {}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        }
}

fn chunk_spawner(commands: &mut Commands, tile_set: Res<TilesetHandle>, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos::new(x, y);
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE as f32,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE as f32,
        0.0,
    ));
    let texture_handle = tile_set.sprite.clone();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize::new(TILE_SIZE as f32, TILE_SIZE as f32),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize::new(TILE_SIZE as f32, TILE_SIZE as f32),
        transform,
        ..Default::default()
    });
}

fn spawn_map(commands: &mut Commands, tileset: Res<TilesetHandle>, asset_server: &AssetServer) {
    chunk_spawner(commands, tileset, IVec2::ZERO);
}