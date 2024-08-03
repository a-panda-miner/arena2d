// This crate handles all items that are spawned in the game world
// and can be picked up

use bevy::prelude::*;
use ar_template::{ItemTemplates, ItemsUtil};
use ar_core::{SpawnItemEvent, PlayerMarker};

pub struct ItemsPlugins;

impl Plugin for ItemsPlugins {
    fn build(&self, app: &mut App) {

    }
}


/// A system that handles the spawning of items in the world
pub fn item_spanwer(
    commands: &mut Commands,
    items: Res<ItemTemplates>,
    mut ev_spawned: EventReader<SpawnItemEvent>,
) {

}