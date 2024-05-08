// This crate defines the monsters, their AI and their spawn logic

pub mod ai;
pub mod spawn;

use ai::{AIPlugin, Chase};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use bevy_xpbd_2d::prelude::*;
use rand_core::RngCore;
use spawn::SpawnPlugin;

use ar_camera::{ARENA_HEIGHT_ZOOMOUT, ARENA_WIDTH_ZOOMOUT};
use ar_core::{
    AppState, BaseSpeed, Cooldown, GameScore, Layer, MinutesSurvived, MonsterMarker, MonsterSet,
    MonstersAlive, PlayerMarker,
};
use ar_enemies::{MonsterLayoutType, MonsterSprites};
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
        .add_plugins(AIPlugin);
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
