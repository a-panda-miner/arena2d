use ar_core::{
    AppState, BoostUsage, Damage, DashUsage, Health, MonsterMarker, MonsterProjectileMarker,
    PlayerDirection, PlayerInvulnerableFrames, PlayerMarker, PlayerProjectileMarker,
};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_2d::{prelude::*, PhysicsSchedule, PhysicsStepSet};

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
        app.add_event::<PlayerDamageEvent>()
            .add_event::<DamageEvent>()
            .add_systems(
                PhysicsSchedule,
                (move_player, handle_collision, player_damaged_handler)
                    .chain()
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

#[derive(Event)]
pub struct DamageEvent {
    pub damage: usize,
    pub target: Entity,
    pub source: Entity,
}

#[derive(Event)]
pub struct PlayerDamageEvent {
    pub damage: usize,
    pub target: Entity,
    pub source: Entity,
}

/// Handles the possible collisions accordinly to Layers' rules,
/// Player only gets damaged by the largest damage source possible
fn handle_collision(
    mut ev_collision_reader: EventReader<Collision>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_player_damage: EventWriter<PlayerDamageEvent>,
    damage: Query<&Damage>,
    monster_query: Query<Entity, With<MonsterMarker>>,
    monster_projectile_query: Query<Entity, With<MonsterProjectileMarker>>,
    player_projectile_query: Query<Entity, With<PlayerProjectileMarker>>,
    player_query: Query<Entity, With<PlayerMarker>>,
) {
    if ev_collision_reader.is_empty() {
        return;
    }
    let mut player_damage = 0;
    let mut source: Entity = Entity::from_raw(0);
    for Collision(collision) in ev_collision_reader.read() {
        let entity1 = collision.entity1;
        let entity2 = collision.entity2;

        if player_query.contains(entity1) {
            if let Ok(damage) = damage.get(entity2) {
                if player_damage < damage.0 {
                    player_damage = damage.0;
                    source = entity2;
                }
            }
        } else if player_query.contains(entity2) {
            if let Ok(damage) = damage.get(entity1) {
                if player_damage < damage.0 {
                    player_damage = damage.0;
                    source = entity1;
                }
            }
        } else if monster_query.contains(entity1) {
            if player_projectile_query.contains(entity2) {
                ev_damage.send(DamageEvent {
                    damage: damage.get(entity2).unwrap().0,
                    target: entity1,
                    source: entity2,
                });
            }
        } else if monster_query.contains(entity2) {
            if player_projectile_query.contains(entity1) {
                ev_damage.send(DamageEvent {
                    damage: damage.get(entity1).unwrap().0,
                    target: entity2,
                    source: entity1,
                });
            }
        } else if (monster_projectile_query.contains(entity1)
            || monster_projectile_query.contains(entity2))
            && (player_projectile_query.contains(entity1)
                || player_projectile_query.contains(entity2))
        {
            ev_damage.send(DamageEvent {
                damage: damage.get(entity1).unwrap().0,
                target: entity1,
                source: entity2,
            });
            ev_damage.send(DamageEvent {
                damage: damage.get(entity2).unwrap().0,
                target: entity2,
                source: entity1,
            });
        }
    }
    if player_damage > 0 {
        ev_player_damage.send(PlayerDamageEvent {
            damage: player_damage,
            target: player_query.single(),
            source,
        });
    }
}

fn player_damaged_handler(
    time: Res<Time>,
    mut ev_damage: EventReader<PlayerDamageEvent>,
    mut player_inv: Query<&mut PlayerInvulnerableFrames, With<PlayerMarker>>,
    mut player_health: Query<&mut Health, With<PlayerMarker>>,
) {
    let mut inv = player_inv.single_mut();
    inv.timer.tick(time.delta());
    if inv.timer.just_finished() || !inv.timer.finished() {
        ev_damage.clear();
        return;
    }
    if ev_damage.is_empty() {
        return;
    }
    let mut player_health = player_health.single_mut();
    for ev in ev_damage.read() {
        player_health.0 -= ev.damage;
    }
    ev_damage.clear();
    inv.timer.reset();
}
