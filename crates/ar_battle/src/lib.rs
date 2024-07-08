use ar_core::{
    AppState, BattleSet, BoostUsage, CollidedHash, Damage, DashUsage, DeathEvent,
    DisplayDamageEvent, Health, Layer, LifeTime, MonsterMarker, MonsterProjectileMarker,
    Penetration, PlayerDirection, PlayerInvulnerableFrames, PlayerLastDirection, PlayerMarker,
    PlayerMinusHpEvent, PlayerProjectileMarker, ProjectilePattern,
};
use ar_spells::generator::{OwnedProjectileSpells, ProjectileSpells};
use avian2d::{prelude::*, schedule::PhysicsSchedule, schedule::PhysicsStepSet};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_asset_loader::prelude::*;

pub struct BattlePlugin;

#[derive(AssetCollection, Resource)]
pub struct SpellsSheetBig {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 2, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spells/dagger32x16.png"), collection(mapped, typed))]
    pub sprite: HashMap<AssetFileStem, Handle<Image>>,
}

#[derive(AssetCollection, Resource)]
pub struct SpellsSheetSmall {
    #[asset(texture_atlas_layout(tile_size_x = 8, tile_size_y = 8, columns = 1, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spells/dagger8x8.png"), collection(mapped, typed))]
    pub sprite: HashMap<AssetFileStem, Handle<Image>>,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamageEvent>()
            .add_event::<PlayerMinusHpEvent>()
            .add_event::<DamageEvent>()
            .add_event::<DisplayDamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                PhysicsSchedule,
                (
                    move_player,
                    handle_collision,
                    player_damaged_handler,
                    damage_applier,
                    death_applier,
                )
                    .chain()
                    .before(PhysicsStepSet::First)
                    .run_if(in_state(AppState::InBattle)),
            )
            .add_systems(
                FixedUpdate,
                (queue_spawn_player_projectiles, spawn_player_projectiles)
                    .chain()
                    .in_set(BattleSet),
            );
    }
}

/// Changes the player's LinearVelocity based on input.
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
// TODO! Check for collided entities with projectiles,
// ignore collisions with entities already collided with the same projectile
fn handle_collision(
    mut ev_collision_reader: EventReader<CollisionStarted>,
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
    for CollisionStarted(entity1, entity2) in ev_collision_reader.read() {
        let entity1 = entity1.clone();
        let entity2 = entity2.clone();

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
    mut ev_player_damaged: EventWriter<PlayerMinusHpEvent>,
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
        if player_health.0 <= ev.damage {
            player_health.0 = 0;
        } else {
            player_health.0 -= ev.damage;
            ev_player_damaged.send(PlayerMinusHpEvent { damage: ev.damage });
        }
    }
    inv.timer.reset();
}

// TODO! This implementation makes all projectiles to spawn at the same time, there should be a delay between them
// TODO! Write the Circle logic
fn queue_spawn_player_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projs: Query<&mut OwnedProjectileSpells, With<PlayerMarker>>,
) {
    if projs.is_empty() {
        return;
    }

    let mut projs = projs.single_mut();
    for proj in projs.spells.iter_mut() {
        if !proj.cooldown.tick(time.delta()).finished() {
            break;
        }
        for i in 0..proj.count {
            let time_to_spawn: f32 = (i as f32 + 1.0) * 2.0 / 3.0
                * (proj.cooldown.duration().as_secs_f32() / proj.count as f32);
            let angle = match proj.pattern {
                ProjectilePattern::Circle => Vec2::ZERO,
                ProjectilePattern::Line => Vec2::ZERO,
            };
            commands.spawn(PlayerProjectileSpawner {
                timer: Timer::from_seconds(time_to_spawn, TimerMode::Once),
                angle: angle,
                spell_name: proj.name.clone(),
            });
        }
    }
}

/// Applies damage to the target,
/// except for the player
fn damage_applier(
    mut commands: Commands,
    mut ev_damage: EventReader<DamageEvent>,
    mut health: Query<&mut Health, Without<PlayerMarker>>,
    mut death_event: EventWriter<DeathEvent>,
    mut display_damage: EventWriter<DisplayDamageEvent>,
    mut player_projectile: Query<(Entity, &mut Penetration), With<PlayerProjectileMarker>>,
) {
    if ev_damage.is_empty() {
        return;
    }
    for ev in ev_damage.read() {
        if let Ok((_, mut pen)) = player_projectile.get_mut(ev.source) {
            if pen.0 == 0 {
                // as the command isn't applied until at least after the end of the function,
                // it is safe to do so
                commands.entity(ev.source).despawn_recursive();
            } else {
                pen.0 -= 1;
            }
        }

        if let Ok(mut health) = health.get_mut(ev.target) {
            if health.0 <= ev.damage {
                death_event.send(DeathEvent { target: ev.target });
            } else {
                health.0 -= ev.damage;
            }
        }
        display_damage.send(DisplayDamageEvent {
            damage: ev.damage,
            target: ev.target,
        });
    }
    ev_damage.clear();
}

fn death_applier(mut commands: Commands, mut ev_death: EventReader<DeathEvent>) {
    if ev_death.is_empty() {
        return;
    }
    for ev in ev_death.read() {
        commands.entity(ev.target).despawn_recursive();
    }
    ev_death.clear();
}

#[derive(Component, Debug)]
struct PlayerProjectileSpawner {
    timer: Timer,
    angle: Vec2,
    spell_name: String,
}

fn spawn_player_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    player_position: Query<(&GlobalTransform, &Transform), With<PlayerMarker>>,
    mut spawner: Query<(Entity, &mut PlayerProjectileSpawner)>,
    sprite_sheet: Res<SpellsSheetSmall>,
    spell: Res<ProjectileSpells>,
    player_last_direction: Res<PlayerLastDirection>,
) {
    if spawner.is_empty() {
        return;
    }
    let player_last_direction = player_last_direction.direction;
    let (player_position, player_transform) = player_position.single();
    for (entity, mut spa) in spawner.iter_mut() {
        if !spa.timer.tick(time.delta()).just_finished() {
            break;
        }
        let global_transform = player_position;
        let local_transform = player_transform;
        let angle = spa.angle + player_last_direction;
        let mut dir = angle.normalize_or_zero();
        if dir == Vec2::ZERO {
            dir = Vec2::X;
        }
        let proj = spell
            .projectile_spells
            .get(&spa.spell_name)
            .expect(format!("{} not found", spa.spell_name).as_str());
        let linear_vel = dir * proj.projectile_movespeed;
        let sprite = proj.sprite.clone();
        commands
            .entity(entity)
            .insert(SpriteBundle {
                texture: sprite_sheet
                    .sprite
                    .get(sprite.as_str())
                    .expect(format!("{} not found", sprite).as_str())
                    .clone()
                    .into(),
                global_transform: *global_transform,
                transform: *local_transform,
                ..Default::default()
            })
            .insert(TextureAtlas {
                layout: sprite_sheet.layout.clone(),
                ..Default::default()
            })
            .insert(PlayerProjectileMarker)
            .insert(RigidBody::Kinematic)
            .insert(Mass(proj.mass))
            .insert(LinearVelocity(linear_vel))
            .insert(AngularVelocity(0.0))
            .insert(Collider::circle(proj.radius))
            .insert(CollisionLayers::new(
                [Layer::PlayerProjectile],
                [Layer::Monster, Layer::MonsterProjectile],
            ))
            .insert(Damage(proj.damage))
            .insert(Penetration(proj.penetration))
            .insert(LifeTime {
                timer: Timer::from_seconds(proj.lifetime, TimerMode::Once),
            })
            .insert(CollidedHash(HashSet::with_capacity(
                proj.penetration.into(),
            )))
            .remove::<PlayerProjectileSpawner>();
    }
}
