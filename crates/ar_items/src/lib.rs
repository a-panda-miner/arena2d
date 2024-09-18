// This crate handles all items that are spawned in the game world
// and can be picked up

use ar_core::{
    DropItemEvent, ItemComponent, ItemMarker, ItemType, ItemsSet, Layer, PickupEvent,
    PlayerExperience, PlayerMarker,
};
use ar_template::items::{ItemTemplates, ItemsUtil};
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

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (item_spawner, pickup_item).in_set(ItemsSet));
    }
}

/// A system that handles the spawning of items in the world
// TODO! Reduce the number of allocations
pub fn item_spawner(
    mut commands: Commands,
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

        let item_random = &table[random];
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
            .insert(Collider::circle(2.0))
            .insert(Mass::from(0.1))
            .insert(RigidBody::Kinematic)
            .insert(CollisionLayers::new(
                [Layer::Item],
                [Layer::Item, Layer::Magnet],
            ))
            .insert(ItemComponent {
                item_type: item.item_type,
                value: item.base_value,
            });
    }
}

pub fn pickup_item(
    mut commands: Commands,
    mut ev_pickup: EventReader<PickupEvent>,
    mut player_experience: Query<&mut PlayerExperience, With<PlayerMarker>>,
    query: Query<&ItemComponent, With<ItemMarker>>,
) {
    let mut player_experience = player_experience.single_mut();
    for ev in ev_pickup.read() {
        if let Ok(item) = query.get(ev.entity) {
            match item.item_type {
                ItemType::ExperienceOrb => {
                    player_experience.0 += item.value;
                }
                _ => {}
            }
        }
        commands.entity(ev.entity).despawn_recursive();
    }
}
