// This crate initiates the resource OneShotSystems
// It needs to be a new crate because of cyclic dependencies,
// this crate should be right before the leaf node (ar_game) on the dependency graph, while the ar_core should be the root node
// for the libraries crates

use ar_core::OneShotSystems;
use ar_input::change_camera_follow_state;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct OneShotPlugin;

impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        let mut systems = OneShotSystems(HashMap::new());

        systems.0.insert(
            "change_camera_follow_state".into(),
            app.register_system(change_camera_follow_state),
        );

        app.insert_resource(systems);
    }
}
