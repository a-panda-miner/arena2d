#[cfg(debug_assertions)]
use ar_core::{PlayerExperience, PlayerMarker, ItemMarker, Layer, ItemComponent};

#[cfg(debug_assertions)]
use ar_template::items::{ItemTemplates, ItemsUtil};

#[cfg(debug_assertions)]
use bevy_rand::prelude::WyRand;

#[cfg(debug_assertions)]
use bevy_rand::resource::GlobalEntropy;

#[cfg(debug_assertions)]
use rand_core::RngCore;

#[cfg(debug_assertions)]
use avian2d::prelude::*;

#[cfg(debug_assertions)]
use ar_items::ItemSheetSmall;

/// This crate adds functions for debugging the game

#[cfg(debug_assertions)]
use bevy::prelude::*;

/// Gives exp to the player, used to test level-up logic
#[cfg(debug_assertions)]
pub fn give_exp(mut player_exp: Query<&mut PlayerExperience, With<PlayerMarker>>) {
    let mut player_exp = player_exp.single_mut();
    player_exp.0 += 50;
}

/// Spawns items
#[cfg(debug_assertions)]
pub fn spawn_item_debug(
    items: Res<ItemTemplates>,
    items_util: Res<ItemsUtil>,
    items_sheet: Res<ItemSheetSmall>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut commands: Commands) {
        let loot_table: u8 = 0;
        let table: Vec<String> = items_util
            .items_names_by_loot_table
            .get(&loot_table)
            .expect("Table not found")
            .clone();
        let table_len = table.len();

        if table_len == 0 {
            return
        }

        let random = (rng.next_u64() as usize) % table_len;

        let item_random = &table[random];
        let item = &items.items.get(item_random).expect("Item not found");

        let position = Vec3::ZERO;

        let sprite = items_sheet
            .sprite
            .get(item.sprite.as_str())
            .expect("Sprite not found");
        let layout = items_sheet.layout.clone();

        commands
            .spawn_empty()
            .insert(ItemMarker)
            .insert(Sprite {
                image: sprite.clone(),
                texture_atlas: Some(layout.clone().into()),
                ..default()
            })
            .insert(Transform::from_translation(position))
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