use ar_core::{AppState, DropType, LoadingTemplatesSet, RewardType, WeaponType};
use ar_enemies::{MonsterAI, MonsterLayoutType, QualityMonster};
use ar_spells::{
    SpellAOEType, SpellBuffType, SpellProjectileType, SpellSummonType, SpellSwingType, SpellType,
};
use bevy::prelude::*;
use bevy::utils::HashMap;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

use std::env;
use std::path::PathBuf;

pub trait FromReader<R> {
    fn from_reader(reader: R) -> Result<Self, ron::error::SpannedError>
    where
        Self: Sized;
}

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::LoadingTemplates),
            (load_templates, cache_templates_info)
                .chain()
                .in_set(LoadingTemplatesSet),
        );
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct MonsterTemplate {
    // The name of the monster
    pub name: String,
    // The sprite of the monster
    pub sprite_name: String,
    pub layout: MonsterLayoutType,
    pub hp: u32,
    pub damage: u32,
    pub movespeed: Option<f32>,
    pub attackspeed: Option<f32>,
    pub regen: Option<u32>,
    pub attack_range: Option<u32>,
    pub rewards: Option<Vec<RewardType>>,
    // The AI of the monster, if none then it just stays still
    pub ai: Option<MonsterAI>,
    // The weapons that the monster can use, if no weapon then it just walks
    // towards the player
    pub weapons: Option<Vec<WeaponType>>,
    // The quality of the monster is used to add special effects to the monster
    // and when it should be spawned
    pub quality: Option<QualityMonster>,
    // Determines what items can be dropped
    pub drops: Option<Vec<DropType>>,
    // Multiplier for the drop chance, the base is tied to DropType then multiplied
    // by this
    pub drops_chance: Option<f32>,
    // Determines how much score is needed for the monster to be added
    // to the spawn pool
    pub difficulty: u32,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct MonsterTemplates {
    pub templates: HashMap<String, MonsterTemplate>,
}

impl FromReader<File> for MonsterTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}

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

pub fn load_templates(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let mut spell_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut monster_path = spell_path.clone();
    spell_path.push("../ar_bin/assets/templates/spells.ron");
    monster_path.push("../ar_bin/assets/templates/monsters.ron");
    let spell_file =
        File::open(spell_path.clone()).expect(&format!("failed to load {:?}", spell_path));
    let monster_file =
        File::open(monster_path.clone()).expect(&format!("failed to load {:?}", monster_path));
    let monstertemplate =
        MonsterTemplates::from_reader(monster_file).expect("failed to parse monsters.ron");
    let spelltemplate =
        SpellTemplates::from_reader(spell_file).expect("failed to parse spells.ron");
    commands.insert_resource(monstertemplate);
    commands.insert_resource(spelltemplate);

    next_state.set(AppState::InBattle);
}

/// Flat list of all monsters in the game, sorted by difficulty
#[derive(Debug, Resource, Clone)]
pub struct MonsterFlatList {
    pub name_difficulty: Vec<(String, u32)>,
}

/// Flat lists of all monsters in the game,
/// excpet for bosses, each list
/// represents a different difficulty
#[derive(Debug, Resource)]
pub struct MonsterDifficultyLists {
    pub difficulty_1: Vec<String>,
    pub difficulty_2: Vec<String>,
    pub difficulty_3: Vec<String>,
    pub difficulty_4: Vec<String>,
}

/// Builds and inserts MonsterFlatList and MonsterDifficultyLists
pub fn cache_templates_info(mut commands: Commands, monstertemplate: Res<MonsterTemplates>) {
    let mut name_difficulty = MonsterFlatList {
        name_difficulty: Vec::new(),
    };
    for (key, template) in monstertemplate.templates.iter() {
        name_difficulty
            .name_difficulty
            .push((key.clone(), template.difficulty));
    }
    name_difficulty
        .name_difficulty
        .sort_by(|a, b| a.1.cmp(&b.1));
    commands.insert_resource(name_difficulty.clone());

    let difficulty_1: Vec<String> = name_difficulty
        .name_difficulty
        .clone()
        .into_iter()
        .filter(|x| x.1 == 1)
        .map(|x| x.0)
        .collect();
    let difficulty_2: Vec<String> = name_difficulty
        .name_difficulty
        .clone()
        .into_iter()
        .filter(|x| x.1 == 2)
        .map(|x| x.0)
        .collect();
    let difficulty_3: Vec<String> = name_difficulty
        .name_difficulty
        .clone()
        .into_iter()
        .filter(|x| x.1 == 3)
        .map(|x| x.0)
        .collect();
    let difficulty_4: Vec<String> = name_difficulty
        .name_difficulty
        .clone()
        .into_iter()
        .filter(|x| x.1 == 4)
        .map(|x| x.0)
        .collect();

    commands.insert_resource(MonsterDifficultyLists {
        difficulty_1,
        difficulty_2,
        difficulty_3,
        difficulty_4,
    });
}
