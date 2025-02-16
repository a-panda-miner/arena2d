use crate::{Resource, SpellTemplates};
use ar_core::{AppState, CardType, CardsTemplates, RemainingCardsByType};
use bevy::prelude::{info, Commands, NextState, Res, ResMut};

#[derive(Resource)]
pub struct CardsByType {
    powerup_cards: Vec<String>,
    spell_cards: Vec<String>,
}

pub fn build_cards_by_type(cards_templates: Res<CardsTemplates>, mut commands: Commands) {
    let mut cards_by_type = CardsByType {
        powerup_cards: Vec::new(),
        spell_cards: Vec::new(),
    };

    for (key, template) in cards_templates.cards.iter() {
        match template.card_type {
            CardType::Buff => {
                cards_by_type.powerup_cards.push(key.clone());
            }
            CardType::Spell => {
                cards_by_type.spell_cards.push(key.clone());
            }
        }
    }
    let remaining_cards_by_type = RemainingCardsByType {
        powerup_cards: cards_by_type.powerup_cards.clone(),
        spell_cards: cards_by_type.spell_cards.clone(),
    };
    commands.insert_resource(cards_by_type);
    commands.insert_resource(remaining_cards_by_type);
}

/// Remove spell cards that reference a spell that doesn't exist,
/// it moves the app to the next stage after validating the spell cards
pub fn validate_spell_cards(
    mut cards_templates: ResMut<CardsTemplates>,
    mut cards_by_type: ResMut<CardsByType>,
    spell_templates: Res<SpellTemplates>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut spell_cards_exist = Vec::new();

    for spell_card in cards_by_type.spell_cards.drain(..) {
        let spell = cards_templates
            .cards
            .get(&spell_card)
            .unwrap()
            .spell
            .clone()
            .unwrap_or("none".to_string());
        if !spell_templates.spells.contains_key(spell.as_str()) {
            info!("Spell not found: {}", spell);
            cards_templates.cards.remove(spell_card.as_str());
        } else {
            spell_cards_exist.push(spell_card);
        }
    }

    cards_by_type.spell_cards = spell_cards_exist;

    next_state.set(AppState::InBattle);
}
