// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_xpbd_2d::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    LoadingTemplates,
    Menu,
    Pause,
    InBattle,
    Shop,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BattleSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemiesSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AISet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingTemplatesSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MonsterSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UtilSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpellSet;

/// The base speed of the entity,
/// Needed for reference when applying and removing slow/stun effects
#[derive(Component)]
pub struct BaseSpeed(pub f32);

#[derive(Component)]
pub struct Stunned;

#[derive(Component)]
pub struct Silenced;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct MonsterMarker;

#[derive(Component)]
pub struct UiMarker;

#[derive(Component)]
pub struct Armor;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct PlayerInvulnerableFrames {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct MaxHealth(pub usize);

#[derive(Component)]
pub struct Damage(pub usize);

#[derive(Component)]
pub struct BGMusicMarker;

#[derive(Event)]
pub struct PlayerMinusHpEvent {
    pub damage: usize,
}

#[derive(Debug, Event)]
pub struct PlayerDirection(pub Vec2);

#[derive(Debug, Event)]
pub struct BoostUsage(pub bool);

#[derive(Debug, Event)]
pub struct DashUsage(pub bool);

#[derive(Debug, Event)]
pub struct ZoomIn;

#[derive(Debug, Event)]
pub struct ZoomOut;

#[derive(Debug, Event)]
pub struct ChangeBackgroundEvent;

#[derive(Clone, Deserialize, Debug)]
pub enum MonsterLayoutType {
    Small,
    Medium,
    Large,
    Boss,
}

#[derive(Clone, Deserialize, Debug)]
pub enum SpellType {
    Summon,
    Projectile,
    Swing,
    Buff,
    AoE,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RewardType {
    Currency,
    SpecialCurrency,
    PetXp,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DropType {
    Currency,
    SpecialCurrency,
    PetXp,
}

#[derive(Clone, Debug, Component, Deserialize)]
pub enum WeaponType {
    Dagger,
}

#[derive(Debug, Component)]
pub struct LifeTime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SummonSpellMarker;

#[derive(Component)]
pub struct AoESpellMarker;

#[derive(Component)]
pub struct ProjectileSpellMarker;

#[derive(Component)]
pub struct SwingSpellMarker;

#[derive(Component)]
pub struct BuffSpellMarker;

#[derive(Component)]
pub struct PlayerProjectileMarker;

#[derive(Component)]
pub struct MonsterProjectileMarker;

#[derive(PhysicsLayer)]
pub enum Layer {
    Player,
    Monster,
    Neutral,
    PlayerProjectile,
    MonsterProjectile,
    Pet,
}

/// Keeps the internal score of the game,
/// used for spawning rules
#[derive(Resource)]
pub struct GameScore(pub usize);

/// Used for calculating GameScore
/// and spawning monsters
#[derive(Resource)]
pub struct MonstersAlive(pub usize);

/// Used for calculating GameScore
/// and spawning bosses
#[derive(Resource)]
pub struct MinutesSurvived(pub usize);

#[derive(Clone, Deserialize, Debug)]
pub enum SpellAOEType {
    Arc,
    Circle,
    Rectangle,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellSummonType {
    pub summon_name: String,
    pub summon_sprite: String,
    pub summon_layout: Option<MonsterLayoutType>,
    pub summon_limit: u8,
    pub summon_hp: u32,
    pub summon_damage: u32,
    pub summon_movespeed: Option<f32>,
    pub summon_attackspeed: Option<f32>,
    pub summon_regen: Option<u32>,
    pub summon_attack_range: Option<u32>,
    pub summon_weapon: Option<WeaponType>,
    // If it is of type None then the summon is permanent
    pub summon_duration: Option<f32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellProjectileType {
    pub projectile_sprite: String,
    // If it is of type None then the projectile is invulnerable
    pub projectile_hp: Option<u32>,
    pub projectile_count: u8,
    pub projectile_pattern: ProjectilePattern,
    pub projectile_damage: usize,
    pub projectile_movespeed: f32,
    pub projectile_radius: f32, // The size of the collider
    pub projectile_mass: f32,   // The mass of the collider
    pub projectile_lifetime: f32,
    pub projectile_bounce: Option<bool>,
    pub projectile_penetration: Option<u8>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellSwingType {
    pub swing_sprite: String,
    pub swing_damage: u32,
    pub swing_arc: f32,
    pub swing_length: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellBuffType {
    pub buff_duration: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub enum ProjectilePattern {
    Circle, // Shoots projectiles in a circle pattern, starting at 360° and reducing by 360°/n for each projectile, where n is projectile_count
    Line,   // Shoots projectiles one after the other in quick succession
}
