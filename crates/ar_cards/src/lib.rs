use ar_core::{
    ApplyCard, CardSet, CardType, CardsTemplates, ChooseACard, ChosenCard, LevelUpEvent, MaxHealth,
    MaxStamina, PlayerMarker, PowerUp, RemainingCardsByType
};
use ar_spells::generator::{OwnedProjectileSpells, OwnedAOESpells, ProjectileSpells, AOESpells};
use bevy::prelude::*;
use bevy_rand::prelude::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::prelude::Rng;
use std::collections::HashSet;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChosenCard>()
            .add_event::<ApplyCard>()
            .init_resource::<ChooseACard>()
            .add_systems(
                FixedUpdate,
                (spawn_cards, chosen_card).chain().in_set(CardSet),
            );
    }
}

fn spawn_cards(
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    cards_by_type: Res<RemainingCardsByType>,
    mut player_level: EventReader<LevelUpEvent>,
    mut choose_a_card: ResMut<ChooseACard>,
) {
    let spell_cards_range = cards_by_type.spell_cards.len();
    let buff_cards_range = cards_by_type.powerup_cards.len();

    let mut cards: [Option<String>; 3];

    for level in player_level.read() {
        let level = level.level;

        #[cfg(debug_assertions)]
        info!("level: {}", level);

        if level % 5 == 0 && spell_cards_range > 0 {
            let mut cards_index = HashSet::new();
            while cards_index.len() < 3 && cards_index.len() < spell_cards_range {
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
            while cards_index.len() < 3 && cards_index.len() < buff_cards_range {
                cards_index.insert(rng.gen_range(0..buff_cards_range));
                #[cfg(debug_assertions)]
                info!(
                    "cards_index_len: {}, buff_cards_range: {}",
                    cards_index.len(),
                    buff_cards_range
                );
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

/// Removes the current available cards from ChooseACard resource and applies the effects
/// of the chosen card to the player
#[allow(clippy::too_many_arguments)]
fn chosen_card(
    mut player: Query<(&mut MaxHealth, &mut MaxStamina), With<PlayerMarker>>,
    mut choose_a_card: ResMut<ChooseACard>,
    mut ev_chosen_card: EventReader<ApplyCard>,
    mut remaining_cards: ResMut<RemainingCardsByType>,
    cards_templates: Res<CardsTemplates>,
    aoe_list: Res<AOESpells>,
    proj_list: Res<ProjectileSpells>,
    mut owned_aoe: Single<&mut OwnedAOESpells>,
    mut owned_projectiles: Single<&mut OwnedProjectileSpells>,
) {
    let (mut player_health, mut player_stamina) = player.single_mut();

    for card in ev_chosen_card.read() {
        choose_a_card.cards.clear();
        let card_template = cards_templates
            .cards
            .get(card.card.as_str())
            .unwrap_or_else(|| panic!("Card doesn't exist: {:?}", card.card));
        match card_template.card_type {
            CardType::Spell => {
                if let Some(spell) = &card_template.spell {
                    if let Some(aoe_spell) = aoe_list.aoe_spells.get(spell) {
                        if !owned_aoe.spells.contains(aoe_spell) {
                            owned_aoe.spells.push(aoe_spell.clone());
                            if let Some(idx) = remaining_cards.spell_cards.iter().position(|v| *v == aoe_spell.name) {
                                remaining_cards.spell_cards.swap_remove(idx);
                            }

                        }
                    } else if let Some(proj_spell) = proj_list.projectile_spells.get(spell) {
                        if !owned_projectiles.spells.contains(proj_spell) {
                            owned_projectiles.spells.push(proj_spell.clone());
                            if let Some(idx) = remaining_cards.spell_cards.iter().position(|v| *v == proj_spell.name) {
                                remaining_cards.spell_cards.swap_remove(idx);
                            }
                        }
                    }
                }
            }
            CardType::Buff => {
                if let Some(power_up) = &card_template.upgrade {
                    match power_up {
                        PowerUp::HealthUp(health) => {
                            info!("Health: {}", health);
                            player_health.0 += *health as usize;
                        }
                        PowerUp::AttackUp(attack) => {
                            info!("Attack: {}", attack);
                        }
                        PowerUp::ShieldUp(shield) => {
                            info!("Shield: {}", shield);
                        }
                        PowerUp::SpeedUp(speed) => {
                            info!("Speed: {}", speed);
                        }
                        PowerUp::LootUp(loot) => {
                            info!("Loot: {}", loot);
                        }
                        PowerUp::DamageUp(damage) => {
                            info!("Damage: {}", damage);
                        }
                        PowerUp::ExpUp(exp) => {
                            info!("Exp: {}", exp);
                        }
                        PowerUp::StaminaUp(stamina) => {
                            info!("Stamina: {}", stamina);
                            player_stamina.0 += *stamina as f32;
                        }
                    }
                }
            }
        }
    }
}
