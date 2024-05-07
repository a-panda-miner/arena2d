// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use bevy::prelude::*;
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
pub struct Armor;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct BGMusicMarker;

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
