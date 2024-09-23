pub mod cards;
pub mod items;
pub mod monsters;
pub mod spells;

use ar_core::{AppState, LoadingTemplatesSet};
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use std::fs::File;

use std::env;
use std::path::PathBuf;

use crate::{
    cards::{build_cards_by_type, validate_spell_cards, CardsTemplates},
    items::{cache_templates_items_info, ItemTemplates},
    monsters::{cache_templates_monsters_info, MonsterTemplates},
    spells::SpellTemplates,
};

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
                build_cards_by_type,
                validate_spell_cards,
            )
                .chain()
                .in_set(LoadingTemplatesSet),
        );
    }
}

pub fn load_templates(mut commands: Commands) {
    let mut spell_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut monster_path = spell_path.clone();
    let mut item_path = spell_path.clone();
    let mut card_path = spell_path.clone();

    spell_path.push("../ar_bin/assets/templates/spells.ron");
    monster_path.push("../ar_bin/assets/templates/monsters.ron");
    item_path.push("../ar_bin/assets/templates/items.ron");
    card_path.push("../ar_bin/assets/templates/cards.ron");

    let spell_file = File::open(spell_path.clone())
        .unwrap_or_else(|_| panic!("failed to load {:?}", spell_path));
    let monster_file = File::open(monster_path.clone())
        .unwrap_or_else(|_| panic!("failed to load {:?}", monster_path));
    let item_file =
        File::open(item_path.clone()).unwrap_or_else(|_| panic!("failed to load {:?}", item_path));
    let card_file =
        File::open(card_path.clone()).unwrap_or_else(|_| panic!("failed to load {:?}", card_path));

    let monstertemplate =
        MonsterTemplates::from_reader(monster_file).expect("failed to parse monsters.ron");
    let spelltemplate =
        SpellTemplates::from_reader(spell_file).expect("failed to parse spells.ron");
    let itemtemplate = ItemTemplates::from_reader(item_file).expect("failed to parse items.ron");
    let cardtemplate = CardsTemplates::from_reader(card_file).expect("failed to parse cards.ron");

    commands.insert_resource(monstertemplate);
    commands.insert_resource(spelltemplate);
    commands.insert_resource(itemtemplate);
    commands.insert_resource(cardtemplate);
}
