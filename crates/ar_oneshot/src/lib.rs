// This crate initiates the resource OneShotSystems
// It needs to be a new crate because of cyclic dependencies,
// this crate should be right before the leaf node (ar_game) on the dependency graph, while the ar_core should be the root node
// for the libraries crates, excluding ar_game

use ar_core::OneShotSystems;
#[cfg(debug_assertions)]
use ar_debug_fn::{give_exp, spawn_item_debug};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct OneShotPlugin;

impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        let mut systems = OneShotSystems(HashMap::new());

        #[cfg(debug_assertions)]
        systems
            .0
            .insert("give_exp".to_string(), app.register_system(give_exp));

        #[cfg(debug_assertions)]
        systems
            .0
            .insert("spawn_item_debug".to_string(), app.register_system(spawn_item_debug));

        app.insert_resource(systems);
    }
}
