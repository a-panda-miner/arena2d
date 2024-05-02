use bevy::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chase);
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
    target: Entity,
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
fn chase() {}