// This crate defines the level-up logic, the leveling table
// and sends events when the player levels up

use ar_core::{
    AppState, LevelSet, LevelTable, LevelUpEvent, PlayerExperience, PlayerLevel, PlayerMarker,
    MAX_LEVEL,
};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpEvent>()
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
    mut ev_levelup: EventWriter<LevelUpEvent>,
    level_table: Res<LevelTable>,
) {
    let (exp, level) = query.single();

    if exp.0 >= level_table.table[level.0 as usize] && level.0 < MAX_LEVEL {
        ev_levelup.send(LevelUpEvent);
    }
}
