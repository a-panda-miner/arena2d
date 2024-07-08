// This crate defines the monsters, their AI, spawn logic, and animation

pub mod ai;
pub mod animation;
pub mod spawn;

use crate::ai::{AIPlugin, Chase};
use crate::animation::MonsterAnimationPlugin;
use avian2d::prelude::*;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use spawn::SpawnPlugin;

use ar_camera::{ARENA_HEIGHT_ZOOMOUT, ARENA_WIDTH_ZOOMOUT};
use ar_core::{
    AppState, BaseSpeed, Cooldown, Damage, GameScore, Health, Layer, MinutesSurvived,
    MonsterLayoutType, MonsterMarker, MonsterMarkerSmall, MonsterSet, MonstersAlive, PlayerMarker,
};
use ar_enemies::MonsterSprites;
use ar_player::PlayerHandler;
use ar_template::{MonsterDifficultyLists, MonsterTemplates};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InBattle),
            monster_setup.in_set(MonsterSet),
        )
        .add_plugins(SpawnPlugin)
        .add_plugins(AIPlugin)
        .add_plugins(MonsterAnimationPlugin);
    }
}

#[derive(Resource)]
pub struct SpawnerTimer(Timer);

/// Sets up resources used for monster spawning
fn monster_setup(mut commands: Commands) {
    let timer = Timer::from_seconds(3.0, TimerMode::Repeating);
    let spawner_timer = SpawnerTimer(timer);
    let minutes_survived = MinutesSurvived(0);
    let monsters_alive = MonstersAlive(0);
    let game_score = GameScore(0);

    commands.insert_resource(minutes_survived);
    commands.insert_resource(monsters_alive);
    commands.insert_resource(game_score);
    commands.insert_resource(spawner_timer);
}
