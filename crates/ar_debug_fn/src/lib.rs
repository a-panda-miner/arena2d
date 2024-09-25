/// This crate adds functions for debugging the game

#[cfg(debug_assertions)]
use bevy::prelude::*;
#[cfg(debug_assertions)]
use ar_core::{PlayerExperience, PlayerMarker};

/// Gives exp to the player, used to test level-up logic
#[cfg(debug_assertions)]
pub fn give_exp(
    mut player_exp: Query<&mut PlayerExperience, With<PlayerMarker>>,
) {
    let mut player_exp = player_exp.single_mut();
    player_exp.0 += 50;
}