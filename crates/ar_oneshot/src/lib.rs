// This crate initiates the resource OneShotSystems
// It needs to be a new crate because of cyclic dependencies,
// this crate should be right before the leaf node (ar_game) on the dependency graph, while the ar_core should be the root node
// for the libraries crates, excluding ar_game

use ar_core::OneShotSystems;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct OneShotPlugin;

impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        let systems = OneShotSystems(HashMap::new());

        app.insert_resource(systems);
    }
}
