use crate::{Component, Deserialize, File, FromReader, HashMap, Resource};
use ron::de::from_reader;

#[derive(Clone, Deserialize, Debug)]
pub struct CardsTemplate {
    pub name: String,
    pub card_type: CardType, // A card can be a power-up or a spell
    pub max_level: u8,
    pub sprite: String,
    pub rarity: CardRarity,
    pub description: String,
    pub upgrade: Option<PowerUp>,
    pub debuff: Option<PermanentDebuff>,
    pub max_level_bonus: Option<PowerUp>,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct CardsTemplates {
    pub templates: HashMap<String, CardsTemplate>,
}

impl FromReader<File> for CardsTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}

#[derive(Deserialize, Debug, Component, Clone, PartialEq, Hash)]
pub enum CardType {
    Buff,
    Spell,
}

#[derive(Deserialize, Debug, Component, Clone, PartialEq, Hash)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    Mythical,
    Legendary,
    Ultimate,
}

#[derive(Deserialize, Debug, Component, Clone)]
pub enum PowerUp {
    HealthUp(u8),
    AttackUp(u8),
    ShieldUp(u8),
    SpeedUp(u8),
    LootUp(u8),
    DamageUp(u8),
    ExpUp(u8),
    StaminaUp(u8),
}

#[derive(Deserialize, Debug, Component, Clone)]
pub enum PermanentDebuff {
    HealthDown(u8),
    AttackDown(u8),
    StaminaDown(u8),
    ExpDown(u8),
    SpeedDown(u8),
}
