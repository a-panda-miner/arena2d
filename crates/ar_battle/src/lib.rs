use ar_core::{
    AppState, BattleSet, BoostUsage, CollidedHash, CurrentStamina, Damage, DashUsage, DeathEvent,
    DisplayDamageEvent, DropItemEvent, DropsChance, Health, Layer, LifeTime, LootTables,
    MagnetMarker, MaxStamina, MonsterMarker, MonsterProjectileMarker, Penetration, PickupEvent,
    PlayerDirection, PlayerInvulnerableFrames, PlayerLastDirection, PlayerMarker,
    PlayerMinusHpEvent, PlayerProjectileMarker, ProjectilePattern, StaminaRegen,
};
use ar_spells::generator::{OwnedProjectileSpells, ProjectileSpells};
use avian2d::{prelude::*, schedule::PhysicsSchedule, schedule::PhysicsStepSet};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_asset_loader::prelude::*;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

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
            .add_event::<DropItemEvent>()
            .add_event::<PickupEvent>()
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
                (
                    queue_spawn_player_projectiles.in_set(BattleSet),
                    spawn_player_projectiles.in_set(BattleSet),
                    regenerate_stamina.in_set(BattleSet),
                )
                    .chain(),
            );
    }
}

/// Changes the player's LinearVelocity based on input.
fn move_player(
    mut q: Query<&mut LinearVelocity, With<PlayerMarker>>,
    mut ev_direction: EventReader<PlayerDirection>,
    mut ev_boost: EventReader<BoostUsage>,
    mut ev_dash: EventReader<DashUsage>,
    mut player_last_direction: ResMut<PlayerLastDirection>,
) {
    let mut linear_vel = q.single_mut();
    // decelerates the player
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
        player_last_direction.direction = direction;
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
    linear_vel.y += direction.y * 10.0 * (2.5 + boost + dash);
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

/// Handles the possible collisions accordingly to Layers' rules,
/// Player only gets damaged by the largest damage source possible
// TODO! Check for collided entities with projectiles,
// ignore collisions with entities already collided with the same projectile
fn handle_collision(
    mut ev_collision_reader: EventReader<CollisionStarted>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_player_damage: EventWriter<PlayerDamageEvent>,
    mut ev_item_pickup: EventWriter<PickupEvent>,
    damage: Query<&Damage>,
    monster_query: Query<Entity, With<MonsterMarker>>,
    monster_projectile_query: Query<Entity, With<MonsterProjectileMarker>>,
    mut player_projectile_query: Query<(Entity, &mut CollidedHash), With<PlayerProjectileMarker>>,
    player_query: Query<Entity, With<PlayerMarker>>,
    magnet_query: Query<Entity, With<MagnetMarker>>,
) {
    if ev_collision_reader.is_empty() {
        return;
    }
    // The largest damage given to the player this frame
    let mut player_damage = 0;
    let mut source: Entity = Entity::from_raw(0);
    for CollisionStarted(entity1, entity2) in ev_collision_reader.read() {
        let entity1 = *entity1;
        let entity2 = *entity2;

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
        } else if magnet_query.contains(entity1) {
            ev_item_pickup.send(PickupEvent { entity: entity2 });
        } else if magnet_query.contains(entity2) {
            ev_item_pickup.send(PickupEvent { entity: entity1 });
        } else if monster_query.contains(entity1) {
            if player_projectile_query.contains(entity2) {
                // Unwrap safety: It is guaranteed to have the entity as we just checked in the 'if'
                let (_, mut collided) = player_projectile_query.get_mut(entity2).unwrap();
                if collided.0.contains(&entity1) {
                    continue;
                } else {
                    collided.0.insert(entity1);
                }
                ev_damage.send(DamageEvent {
                    damage: damage.get(entity2).unwrap().0,
                    target: entity1,
                    source: entity2,
                });
            }
        } else if monster_query.contains(entity2) {
            if player_projectile_query.contains(entity1) {
                // Unwrap safety: It is guaranteed to have the entity as we just checked in the 'if'
                let (_, mut collided) = player_projectile_query.get_mut(entity1).unwrap();
                if collided.0.contains(&entity2) {
                    continue;
                } else {
                    collided.0.insert(entity2);
                }
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
    mut projectiles: Query<&mut OwnedProjectileSpells, With<PlayerMarker>>,
) {
    if projectiles.is_empty() {
        return;
    }

    let mut projectiles = projectiles.single_mut();
    for proj in projectiles.spells.iter_mut() {
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
                angle,
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
    let mut despawned_projectiles = HashSet::new();
    for ev in ev_damage.read() {
        if despawned_projectiles.contains(&ev.source) {
            continue;
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
        if let Ok((projectile_id, mut pen)) = player_projectile.get_mut(ev.source) {
            if pen.0 == 0 {
                // as the command isn't applied until at least after the end of the function,
                // it is safe to do so
                commands.entity(ev.source).despawn_recursive();

                // flags the entity, so it can't do any more damage this frame
                despawned_projectiles.insert(projectile_id);
            } else {
                pen.0 -= 1;
            }
        }
    }
    ev_damage.clear();
}

fn death_applier(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    item_query: Query<(&GlobalTransform, &LootTables, &DropsChance), With<MonsterMarker>>,
    mut ev_itemdrop: EventWriter<DropItemEvent>,
    mut ev_death: EventReader<DeathEvent>,
) {
    if ev_death.is_empty() {
        return;
    }
    for ev in ev_death.read() {
        if let Ok((transform, table, chance)) = item_query.get(ev.target) {
            for i in 0..table.0.len() {
                let random_number = rng.next_u32() % 100;
                let chance = (chance.0 * 25.0).round() as u32;
                if random_number <= chance {
                    ev_itemdrop.send(DropItemEvent {
                        position: transform.translation(),
                        loot_table: table.0[i],
                    });
                }
            }
        }
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
    player_position: Query<&Transform, With<PlayerMarker>>,
    mut spawner: Query<(Entity, &mut PlayerProjectileSpawner)>,
    sprite_sheet: Res<SpellsSheetSmall>,
    spell: Res<ProjectileSpells>,
    player_last_direction: Res<PlayerLastDirection>,
) {
    info!("Spawning player projectiles");
    if spawner.is_empty() {
        return;
    }
    let player_last_direction = player_last_direction.direction;
    let player_transform = player_position.single();
    for (entity, mut spa) in spawner.iter_mut() {
        if !spa.timer.tick(time.delta()).just_finished() {
            break;
        }
        let local_transform = player_transform;
        let angle = spa.angle + player_last_direction;
        let mut dir = angle.normalize_or_zero();
        if dir == Vec2::ZERO {
            dir = Vec2::X;
        }
        let proj = spell
            .projectile_spells
            .get(&spa.spell_name)
            .unwrap_or_else(|| panic!("{} not found", spa.spell_name));
        let linear_vel = dir * proj.projectile_movespeed;
        let sprite = proj.sprite.clone();
        commands
            .entity(entity)
            .insert(Sprite {
                image: sprite_sheet
                .sprite
                .get(sprite.as_str())
                .unwrap_or_else(|| panic!("{} not found", sprite))
                .clone(),
                texture_atlas: Some(sprite_sheet.layout.clone().into()),
                ..Default::default()
            })
            .insert(*local_transform)
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

fn regenerate_stamina(
    mut stamina_query: Query<(&mut CurrentStamina, &MaxStamina, &StaminaRegen)>,
    time: Res<Time>,
) {
    for (mut stamina, max_stamina, regen) in stamina_query.iter_mut() {
        if stamina.0 < max_stamina.0 {
            stamina.0 = (stamina.0 + regen.0 * time.delta().as_secs_f32()).max(max_stamina.0);
        }
    }
}
