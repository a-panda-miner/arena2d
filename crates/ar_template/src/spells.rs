use crate::{Deserialize, File, FromReader, HashMap, Resource};
use ar_core::{
    SpellAOEType, SpellBuffType, SpellProjectileExplosiveType, SpellProjectileType,
    SpellSummonType, SpellSwingType, SpellType,
};
use ron::de::from_reader;

#[derive(Clone, Deserialize, Debug)]
pub struct SpellTemplate {
    pub name: String,
    pub cooldown: f32,
    pub spell_main_type: SpellType,
    pub spell_sub_types: Option<Vec<SpellType>>,
    pub summon_type_struct: Option<SpellSummonType>,
    pub projectile_type_struct: Option<SpellProjectileType>,
    pub swing_type_struct: Option<SpellSwingType>,
    pub buff_type_struct: Option<SpellBuffType>,
    pub aoe_type_struct: Option<SpellAOEType>,
    pub explosive_projectile_struct: Option<SpellProjectileExplosiveType>,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct SpellTemplates {
    pub spells: HashMap<String, SpellTemplate>,
}

impl FromReader<File> for SpellTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}
