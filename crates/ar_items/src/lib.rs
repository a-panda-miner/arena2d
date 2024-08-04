// This crate handles all items that are spawned in the game world
// and can be picked up

use ar_core::{DropItemEvent, ItemMarker, Layer};
use ar_template::{ItemTemplates, ItemsUtil};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

#[derive(AssetCollection, Resource)]
pub struct ItemSheetSmall {
    #[asset(texture_atlas_layout(tile_size_x = 8, tile_size_y = 8, columns = 1, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "items/exporb.png",
            "items/diamond8x8.png",
            "items/fruit.png",
            "items/ore.png",
        ),
        collection(mapped, typed)
    )]
    pub sprite: HashMap<AssetFileStem, Handle<Image>>,
}

pub struct ItemsPlugins;

impl Plugin for ItemsPlugins {
    fn build(&self, app: &mut App) {}
}

/// A system that handles the spawning of items in the world
// TODO! Reduce the number of allocations
pub fn item_spanwer(
    commands: &mut Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    items: Res<ItemTemplates>,
    items_util: Res<ItemsUtil>,
    items_sheet: Res<ItemSheetSmall>,
    mut ev_spawned: EventReader<DropItemEvent>,
) {
    for ev in ev_spawned.read() {
        let loot_table: u8 = ev.loot_table.0;
        let table: Vec<String> = items_util
            .items_names_by_loot_table
            .get(&loot_table)
            .expect("Table not found")
            .clone();
        let table_len = table.len();
        if table_len == 0 {
            continue;
        }
        let random = (rng.next_u64() as usize) % table_len;

        let item_random = &table[random as usize];
        let item = &items.items.get(item_random).expect("Item not found");

        let position = ev.position;

        let sprite = items_sheet
            .sprite
            .get(item.sprite.as_str())
            .expect("Sprite not found");
        let layout = items_sheet.layout.clone();

        commands
            .spawn_empty()
            .insert(ItemMarker)
            .insert(SpriteBundle {
                texture: sprite.clone(),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(TextureAtlas {
                layout,
                ..Default::default()
            })
            .insert(Collider::circle(0.5))
            .insert(RigidBody::Kinematic)
            .insert(CollisionLayers::new([Layer::Item], [Layer::Item]));
    }
}
