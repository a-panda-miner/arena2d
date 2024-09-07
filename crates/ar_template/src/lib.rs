use ar_core::{
    AppState, ItemType, LoadingTemplatesSet, MonsterLayoutType, RewardType, SpellAOEType,
    SpellBuffType, SpellProjectileExplosiveType, SpellProjectileType, SpellSummonType,
    SpellSwingType, SpellType, WeaponType,
};
use ar_enemies::{MonsterAI, QualityMonster};
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
            (
                load_templates,
                (cache_templates_monsters_info, cache_templates_items_info),
            )
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
    pub hp: usize,
    pub damage: usize,
    pub movespeed: Option<f32>,
    pub attackspeed: Option<f32>,
    pub regen: Option<usize>,
    pub attack_range: Option<f32>,
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
    pub loot_tables: Vec<u8>,
    // Multiplier for the drop chance, the base is tied to DropType then multiplied
    // by this
    pub drops_chance: Option<f32>,
    // Determines how much score is needed for the monster to be added
    // to the spawn pool
    pub difficulty: usize,
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

/// The template of an item,
/// used to deserialize the items from .ron file
/// and for spawning items
#[derive(Clone, Deserialize, Debug)]
pub struct ItemTemplate {
    pub name: String,
    pub item_type: ItemType,
    pub sprite: String,
    pub loot_table: u8,
    pub unique: bool,
    pub base_value: usize,
}

#[derive(Resource, Clone, Deserialize, Debug)]
pub struct ItemTemplates {
    pub items: HashMap<String, ItemTemplate>,
}

/// A resource that contains all items loaded from the .ron,
/// both in a flat way and organized by the loot table's number
#[derive(Resource, Clone, Deserialize, Debug)]
pub struct ItemsUtil {
    pub item_names_flat: Vec<String>,
    pub items_names_by_loot_table: HashMap<u8, Vec<String>>,
}

impl FromReader<File> for ItemTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}

pub fn load_templates(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let mut spell_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut monster_path = spell_path.clone();
    let mut item_path = spell_path.clone();

    spell_path.push("../ar_bin/assets/templates/spells.ron");
    monster_path.push("../ar_bin/assets/templates/monsters.ron");
    item_path.push("../ar_bin/assets/templates/items.ron");

    let spell_file =
        File::open(spell_path.clone()).expect(&format!("failed to load {:?}", spell_path));
    let monster_file =
        File::open(monster_path.clone()).expect(&format!("failed to load {:?}", monster_path));
    let item_file =
        File::open(item_path.clone()).expect(&format!("failed to load {:?}", item_path));

    let monstertemplate =
        MonsterTemplates::from_reader(monster_file).expect("failed to parse monsters.ron");
    let spelltemplate =
        SpellTemplates::from_reader(spell_file).expect("failed to parse spells.ron");
    let itemtemplate = ItemTemplates::from_reader(item_file).expect("failed to parse items.ron");

    commands.insert_resource(monstertemplate);
    commands.insert_resource(spelltemplate);
    commands.insert_resource(itemtemplate);

    next_state.set(AppState::InBattle);
}

/// Flat list of all monsters in the game, sorted by difficulty
#[derive(Debug, Resource, Clone)]
pub struct MonsterFlatList {
    pub name_difficulty: Vec<(String, usize)>,
}

/// Flat lists of all monsters in the game,
/// except for bosses, each list
/// represents a different difficulty
#[derive(Debug, Resource)]
pub struct MonsterDifficultyLists {
    pub difficulty_1: Vec<String>,
    pub difficulty_2: Vec<String>,
    pub difficulty_3: Vec<String>,
    pub difficulty_4: Vec<String>,
}

/// Builds and inserts MonsterFlatList and MonsterDifficultyLists
pub fn cache_templates_monsters_info(
    mut commands: Commands,
    monstertemplate: Res<MonsterTemplates>,
) {
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

fn cache_templates_items_info(mut commands: Commands, itemtemplate: Res<ItemTemplates>) {
    let mut item_names_flat = Vec::new();
    let mut items_names_by_loot_table = HashMap::new();
    for (key, template) in itemtemplate.items.iter() {
        item_names_flat.push(key.clone());
        items_names_by_loot_table
            .entry(template.loot_table)
            .or_insert(Vec::new())
            .push(key.clone());
    }
    item_names_flat.sort();

    let items_util = ItemsUtil {
        item_names_flat,
        items_names_by_loot_table,
    };

    commands.insert_resource(items_util);
}
