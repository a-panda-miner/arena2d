// This crate defines the level-up logic, the leveling table
// and sends events when the player levels up

use ar_core::{
    AppState, AvailableCards, LevelSet, LevelTable, LevelUpEvent, OneShotSystems, PlayerExperience,
    PlayerLevel, PlayerMarker, MAX_LEVEL,
};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpEvent>()
            .insert_resource(AvailableCards(0))
            .add_systems(
                OnEnter(AppState::InBattle),
                setup_generate_level_table.in_set(LevelSet),
            )
            .add_systems(FixedUpdate, check_for_level_up.in_set(LevelSet));
    }
}

fn setup_generate_level_table(mut commands: Commands) {
    let mut total: f32 = 0.0;

    let mut table = [0; MAX_LEVEL as usize];

    let mut next_level: f32 = 100.0;

    let factor: f32 = 0.45;

    for i in 0..MAX_LEVEL as usize {
        total += next_level;
        table[i] = total as usize;
        next_level = next_level * (1.0 + factor.powi(i as i32));
    }

    commands.insert_resource(LevelTable { table });
}

fn check_for_level_up(
    query: Query<(&PlayerExperience, &PlayerLevel), With<PlayerMarker>>,
    level_up_system: Res<OneShotSystems>,
    level_table: Res<LevelTable>,
    mut commands: Commands,
) {
    let (exp, level) = query.single();
    let level_up_system = level_up_system
        .0
        .get("level_up")
        .expect("level_up system not registered as OneShot system!!!");

    if exp.0 >= level_table.table[level.0 as usize] && level.0 < MAX_LEVEL {
        commands.run_system(*level_up_system);
    }
}

pub fn level_up(
    mut ev_levelup: EventReader<LevelUpEvent>,
    mut query: Query<(&mut PlayerLevel, &mut PlayerExperience), With<PlayerMarker>>,
    level_table: Res<LevelTable>,
    mut available_cards: ResMut<AvailableCards>,
) {
    if ev_levelup.is_empty() {
        return;
    }

    for _ in ev_levelup.read() {
        let (mut level, mut experience) = query.single_mut();
        experience.0 -= level_table.table[level.0 as usize];
        level.0 += 1;
    }
    ev_levelup.clear();
    available_cards.0 += 1;
}
