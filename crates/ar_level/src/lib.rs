// This crate defines the level-up logic, the leveling table
// and sends events when the player levels up

use ar_core::{
    AppState, ApplyCard, AvailableCards, ChooseACard, ChosenCard, LevelSet, LevelTable,
    LevelUpEvent, PlayerExperience, PlayerLevel, PlayerMarker, MAX_LEVEL,
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
            .add_systems(
                FixedUpdate,
                (
                    check_for_level_up.in_set(LevelSet),
                    level_up.in_set(LevelSet),
                    choosing_card.in_set(LevelSet),
                )
                    .chain(),
            );
    }
}

fn setup_generate_level_table(mut commands: Commands) {
    let mut total: f32 = 0.0;

    let mut table = [0; MAX_LEVEL as usize];

    let mut next_level: f32 = 100.0;

    let factor: f32 = 0.45;

    for (i, item) in table.iter_mut().enumerate() {
        total += next_level;
        *item = total as usize;
        next_level *= 1.0 + factor.powi(i as i32);
    }

    commands.insert_resource(LevelTable { table });
}

fn check_for_level_up(
    mut query: Query<(&mut PlayerExperience, &mut PlayerLevel), With<PlayerMarker>>,
    level_table: Res<LevelTable>,
    mut ev_levelup: EventWriter<LevelUpEvent>,
) {
    let (mut exp, mut level) = query.single_mut();
    if exp.0 >= level_table.table[level.0 as usize] && level.0 < MAX_LEVEL {
        exp.0 -= level_table.table[level.0 as usize];
        level.0 += 1;
        ev_levelup.send(LevelUpEvent { level: level.0 });
    }
}

pub fn level_up(
    mut ev_levelup: EventReader<LevelUpEvent>,
    mut available_cards: ResMut<AvailableCards>,
) {
    for _ in ev_levelup.read() {
        available_cards.0 += 1;
    }
}

// Checks if the chosen card is valid and sends an event
// with its name to apply the effects of the card
pub fn choosing_card(
    mut ev_choosing_card: EventReader<ChosenCard>,
    mut ev_card_name: EventWriter<ApplyCard>,
    available_cards: Res<AvailableCards>,
    choose_card: Res<ChooseACard>,
) {
    if available_cards.0 == 0 {
        return;
    }
    for card_number in ev_choosing_card.read() {
        if card_number.0 > 2 {
            info!("Invalid card number");
            return;
        }
        let index = card_number.0 as usize;
        if let Some(card) = &choose_card.cards[0][index] {
            ev_card_name.send(ApplyCard {
                card: card.to_string(),
            });
        }
    }
}
