use bevy::prelude::*;

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

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Chase {
    target: Entity,
    speed: f32,
}

// Determines the direction the monster should be chasing the player,
// The movement itself is applied by another system reading the 
// ChaseDirection event
fn chase() {}