// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use serde::Deserialize;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

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
pub struct MapSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingTemplatesSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MonsterSet;

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
pub struct InvFrames {
    frames_remaining: u8,
}

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct BGMusicMarker;

#[derive(Component)]
pub struct Dash {
    max_dashes: u8,
    cooldown: u8, // in frames
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
pub struct ChaseDirection(IVec2);

#[derive(Debug, Event)]
pub struct ChangeBackgroundEvent;

#[derive(Clone, Debug, Deserialize)]
pub enum RewardType {
    Currency,
    SpecialCurrency,
    PetXp
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