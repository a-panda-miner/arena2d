use crate::{Commands, Deserialize, File, FromReader, HashMap, Res, Resource};
use ar_core::ItemType;
use ron::de::from_reader;

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

pub fn cache_templates_items_info(mut commands: Commands, itemtemplate: Res<ItemTemplates>) {
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
