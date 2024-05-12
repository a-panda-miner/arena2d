// Crate for util functions, i.e., despawing entities

use ar_core::{LifeTime, UtilSet};
use bevy::prelude::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn.in_set(UtilSet));
    }
}

fn despawn(time: Res<Time>, mut commands: Commands, mut lifetime: Query<(Entity, &mut LifeTime)>) {
    for (entity, mut lifetime) in lifetime.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
