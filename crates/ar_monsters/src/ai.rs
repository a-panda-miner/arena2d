use crate::{BaseSpeed, LinearVelocity, PlayerMarker};
use ar_core::AISet;
use bevy::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, chase.in_set(AISet));
    }
}

/// All possible actions a monster can take,
/// actions are issued by a transition system or a behavior tree system
#[derive(Clone, Debug)]
pub enum Action {
    Chase,
    Retreat,
    Charge,
    Channeling,
    Idle,
    RandomWalk,
    CircleAround,
}

/// The target of the chaser
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Chase {
    pub target: Entity,
}

/// Representation of the possible states a monster with only
/// chasing and charge attacks can be in
#[derive(Clone, Component, Reflect, Debug)]
pub enum ChasingBehavior {
    Chasing,
    Immovable, // Due to stun
    Charging,
}

/// Adjusts the speed and direction the monster should be moving towards its target
fn chase(
    mut query: Query<(&GlobalTransform, &BaseSpeed, &mut LinearVelocity), With<Chase>>,
    player: Query<&GlobalTransform, With<PlayerMarker>>,
) {
    let player_position = player.single();
    for (transform, base_speed, mut velocity) in query.iter_mut() {
        let speed = (player_position.translation() - transform.translation()).normalize_or_zero()
            * base_speed.0;
        velocity.x = speed.x;
        velocity.y = speed.y;
    }
}
