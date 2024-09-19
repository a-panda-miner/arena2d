use ar_core::{CardSet, PlayerLevelUpEvent};
use ar_template::cards::RemainingCardsByType;
use bevy::prelude::*;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::prelude::Rng;
use std::collections::HashSet;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerLevelUpEvent>()
            .init_resource::<ChooseACard>()
            .add_systems(FixedUpdate, spawn_cards.in_set(CardSet));
    }
}

/// Every time the player levels up, spawn 3 new cards,
/// every level is an index of the array, the player chooses always from the index 0 and then the vec is shifted,
/// If there are no valid remaining cards it will be a None in the array,
/// TODO! if the vec[0] is [None, None, None] grants the player resource and consume the array.
/// If the vec is empty it means the player can't currently choose a new card.
#[derive(Resource, Debug, Default)]
pub struct ChooseACard {
    pub cards: Vec<[Option<String>; 3]>,
}

fn spawn_cards(
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    cards_by_type: Res<RemainingCardsByType>,
    mut player_level: EventReader<PlayerLevelUpEvent>,
    mut choose_a_card: ResMut<ChooseACard>,
) {
    let spell_cards_range = cards_by_type.spell_cards.len();
    let buff_cards_range = cards_by_type.powerup_cards.len();

    let mut cards: [Option<String>; 3];

    for level in player_level.read() {
        if level.0 % 5 == 0 {
            let mut cards_index = HashSet::new();
            while cards_index.len() < 3 || cards_index.len() <= spell_cards_range {
                cards_index.insert(rng.gen_range(0..spell_cards_range));
            }
            let cards_index: Vec<usize> = cards_index.into_iter().collect();
            match cards_index.len() {
                3 => {
                    cards = [
                        Some(cards_by_type.spell_cards[cards_index[0]].clone()),
                        Some(cards_by_type.spell_cards[cards_index[1]].clone()),
                        Some(cards_by_type.spell_cards[cards_index[2]].clone()),
                    ];
                }
                2 => {
                    cards = [
                        Some(cards_by_type.spell_cards[cards_index[0]].clone()),
                        Some(cards_by_type.spell_cards[cards_index[1]].clone()),
                        None,
                    ];
                }
                1 => {
                    cards = [
                        Some(cards_by_type.spell_cards[cards_index[0]].clone()),
                        None,
                        None,
                    ];
                }
                _ => {
                    cards = [None, None, None];
                }
            }
        } else {
            let mut cards_index = HashSet::new();
            while cards_index.len() < 3 || cards_index.len() <= buff_cards_range {
                cards_index.insert(rng.gen_range(0..buff_cards_range));
            }
            let cards_index: Vec<usize> = cards_index.into_iter().collect();
            match cards_index.len() {
                3 => {
                    cards = [
                        Some(cards_by_type.powerup_cards[cards_index[0]].clone()),
                        Some(cards_by_type.powerup_cards[cards_index[1]].clone()),
                        Some(cards_by_type.powerup_cards[cards_index[2]].clone()),
                    ];
                }
                2 => {
                    cards = [
                        Some(cards_by_type.powerup_cards[cards_index[0]].clone()),
                        Some(cards_by_type.powerup_cards[cards_index[1]].clone()),
                        None,
                    ];
                }
                1 => {
                    cards = [
                        Some(cards_by_type.powerup_cards[cards_index[0]].clone()),
                        None,
                        None,
                    ];
                }
                _ => {
                    cards = [None, None, None];
                }
            }
        }
        choose_a_card.cards.push(cards)
    }
}
