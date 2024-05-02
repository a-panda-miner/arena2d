use bevy::prelude::*;
use bevy_xpbd_2d::{prelude::*, PhysicsSchedule, PhysicsStepSet};
use bevy_asset_loader::prelude::*;
use bevy::utils::HashMap;
use ar_core::{AppState, BoostUsage, DashUsage, PlayerDirection, PlayerMarker};

pub struct BattlePlugin;


#[derive(AssetCollection, Resource)]
pub struct SpellsSheetBig {
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spells/dagger32x16.png"), collection(mapped, typed))]
    pub sprite: HashMap<AssetFileStem, Handle<Image>>,
}

#[derive(AssetCollection, Resource)]
pub struct SpellsSheetSmall {
    #[asset(texture_atlas_layout(tile_size_x = 8., tile_size_y = 8., columns = 1, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spells/dagger8x8.png"), collection(mapped, typed))]
    pub sprite: HashMap<AssetFileStem, Handle<Image>>,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            move_player
                .before(PhysicsStepSet::BroadPhase)
                .run_if(in_state(AppState::InBattle)),
        );
    }
}

// Changes the player's LinearVelocity based on input
fn move_player(
    mut q: Query<&mut LinearVelocity, With<PlayerMarker>>,
    mut ev_direction: EventReader<PlayerDirection>,
    mut ev_boost: EventReader<BoostUsage>,
    mut ev_dash: EventReader<DashUsage>,
) {
    let mut linear_vel = q.single_mut();
    // deaccelerates the player
    linear_vel.x *= 0.90;
    linear_vel.y *= 0.90;
    let direction: Vec2;
    let mut boost: f32 = 0.0;
    let mut dash: f32 = 0.0;
    if ev_direction.is_empty() {
        return;
    } else {
        // unwrap safety: the event is guaranteed to have at least 1 element
        direction = ev_direction.read().next().unwrap().0;
        ev_direction.clear();
    }
    if !ev_boost.is_empty() {
        boost = 5.;
    }
    if !ev_dash.is_empty() {
        dash = 5.;
    }
    ev_boost.clear();
    ev_dash.clear();
    linear_vel.x += direction.x * 10.0 * (2.5 + boost + dash);
    //if linear_vel.x > max_vel { linear_vel.x = max_vel;}
    //if linear_vel.x < (-1. * max_vel) { linear_vel.x = -1.0 * max_vel; }
    linear_vel.y += direction.y * 10.0 * (2.5 + boost + dash);
    //if linear_vel.y > max_vel { linear_vel.y = max_vel;}
    //if linear_vel.y < (-1. * max_vel) { linear_vel.y = -1.0 * max_vel; }
}