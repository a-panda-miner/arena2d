// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use avian2d::prelude::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use serde::Deserialize;

/// Defines tha main states of the app
/// LoadingAssets loads the assets during the App startup,
/// LoadingTemplates initiates some of those assets into resources,
/// InitialScreen is the main menu before starting the game (not implemented)
/// InBattle is the main game state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    LoadingTemplates,
    Setup,
    InitialScreen,
    InBattle,
}

/// Different states of pause,
/// Paused is a manual pause of the game, (not implemented)
/// Menu is in the pause menu (not implemented)
/// Shop is triggered by a shop NPC (not implemented)
/// PowerUp is the selection of a power up after reaching a new level (not implemented)
/// MetaUpgrades is the buying of meta upgrades after dying (not implemented)
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PauseState {
    #[default]
    NotPaused,
    Paused,
    Menu,
    Shop,
    PowerUp,
    MetaUpgrades,
}

/// These systems are One-Shot systems, they are ran by calling commands.run_system(id)
#[derive(Resource, Debug)]
pub struct OneShotSystems(pub HashMap<String, SystemId>);

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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleSet;

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

/// The marker for monsters with small layout,
/// used for animation purposes
#[derive(Component)]
pub struct MonsterMarkerSmall;

/// A hashset containing entities' ID that the projectile has collided with already,
/// so there are no collisions events happening multiple times when a projectile collides
/// with a monster/object
/// Performance concerns:
/// to avoid extra allocations initiate with 'with_capacity(n)', where n is the projectile_penetration number
#[derive(Component)]
pub struct CollidedHash(pub HashSet<Entity>);

/// The number of times the projectile can hit different monsters before despawning
#[derive(Component)]
pub struct Penetration(pub u8);

#[derive(Component)]
pub struct UiMarker;

/// Armor reduces damage taken
#[derive(Component)]
pub struct Armor;

/// Shield prevents damage
#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct Cooldown(pub Timer);

/// The timer must be set on Once, and then reset when the player takes damage or
/// is in an invulnerable state, when the timer finishes the player can take damage
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

/// The marker for the background music
#[derive(Component)]
pub struct BGMusicMarker;

/// An event that is triggered when the player is hit,
/// not used yet for player's death
#[derive(Event)]
pub struct PlayerMinusHpEvent {
    pub damage: usize,
}

/// The direction in which the player will move
#[derive(Debug, Event)]
pub struct PlayerDirection(pub Vec2);

/// Boosts the movespeed of the player
#[derive(Debug, Event)]
pub struct BoostUsage(pub bool);

/// Used for moving the player in an dash event,
/// not implemented
#[derive(Debug, Event)]
pub struct DashUsage(pub bool);

/// Zooms in the camera,
/// the camera zoom levels is predetermined in the camera plugin
#[derive(Debug, Event)]
pub struct ZoomIn;

/// Zooms out the camera,
/// the camera zoom levels is predetermined in the camera plugin
#[derive(Debug, Event)]
pub struct ZoomOut;

/// Changes the camera following strategy of the game
/// Rect is the default, adjusts itself only when the player moves out of the current 'rect'
/// Player moves the camera to follow the player each frame
#[derive(Resource, Default, Debug, PartialEq)]
pub enum CameraFollowState {
    #[default]
    Rect,
    Player,
}

/// Changes the background music of the game
#[derive(Debug, Event)]
pub struct ChangeBackgroundEvent;

/// An event that is triggered when the target reaches 0 HP,
/// despawning it and applying the death animation and rewards to
/// the player.
/// The player's death is not handled by this event
#[derive(Debug, Event)]
pub struct DeathEvent {
    pub target: Entity,
}

/// Displays the damage that happened on the ground,
/// damage done to the player is not handled by this event
#[derive(Debug, Event)]
pub struct DisplayDamageEvent {
    pub damage: usize,
    pub target: Entity,
}

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

/// Reward is automatically given when a condition is met
#[derive(Clone, Debug, Deserialize)]
pub enum RewardType {
    Currency,
    SpecialCurrency,
    PetXp,
}

/// Drop is dropped on the ground and the player must pick it up
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

/// Used for spawning projectiles and dashing from where the player was last facing
#[derive(Resource)]
pub struct PlayerLastDirection {
    pub direction: Vec2,
}

/// Defines the cooldown of the dash and if the player is dashing
#[derive(Resource)]
pub struct PlayerDash {
    pub timer: Timer,
    pub cooldown_reduction: f32, // 0.0 - 1.0
    pub dashing: bool,
}

/// The max stamina the entity can have
#[derive(Component)]
pub struct MaxStamina(pub f32);

/// The current stamina of the entity
#[derive(Component)]
pub struct CurrentStamina(pub f32);

/// Regenerates stamina over time
#[derive(Component)]
pub struct StaminaRegen(pub f32);

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
