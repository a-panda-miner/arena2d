use crate::{Deserialize, File, FromReader, HashMap, Resource};
use ar_core::{CardType, CardRarity, PowerUp, PermanentDebuff};
use ron::de::from_reader;
use bevy::prelude::{Res, Commands};

// Note: Spell cards need to verify if the spell exists
#[derive(Clone, Deserialize, Debug)]
pub struct CardsTemplate {
    pub name: String,
    pub card_type: CardType, // A card can be a power-up or a spell
    pub max_level: u8,
    pub sprite: String,
    pub rarity: CardRarity,
    pub description: String,
    pub upgrade: Option<PowerUp>,
    pub spell: Option<String>,
    pub debuff: Option<PermanentDebuff>,
    pub max_level_bonus: Option<PowerUp>,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct CardsTemplates {
    pub cards: HashMap<String, CardsTemplate>,
}

impl FromReader<File> for CardsTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}

#[derive(Resource)]
struct CardsByType {
    powerup_cards: Vec<String>,
    spell_cards: Vec<String>,
}

pub fn templates_cards_by_type(
    templates: Res<CardsTemplates>,
    mut commands: Commands,
) {
    let mut cards_by_type = CardsByType {
        powerup_cards: Vec::new(),
        spell_cards: Vec::new(),
    };
    for (key, template) in templates.cards.iter() {
        match template.card_type {
            CardType::Buff => {
                cards_by_type
                    .powerup_cards
                    .push(key.clone());
            }
            CardType::Spell => {
                cards_by_type
                    .spell_cards
                    .push(key.clone());
            }
        }
    }
    commands.insert_resource(cards_by_type);
}