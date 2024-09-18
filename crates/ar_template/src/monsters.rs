use crate::{Commands, Deserialize, File, FromReader, HashMap, Res, Resource};
use ar_core::{MonsterLayoutType, RewardType, WeaponType};
use ron::de::from_reader;

use ar_enemies::{MonsterAI, QualityMonster};

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
