// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
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

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct Armor;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct InvFrames {
    frames_remaining: u8,
}


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