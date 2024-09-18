pub mod generator;

use crate::generator::GenerateSpellsPlugin;
use ar_core::{AppState, ProjectilePattern, SpellAOEType, SpellSet, SpellType};
use ar_template::spells::SpellTemplates;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

#[derive(Debug, AssetCollection, Resource)]
pub struct SpellSprites {
    #[asset(paths("spells/dagger.png"), collection(mapped, typed))]
    pub spells_sheet: HashMap<AssetFileStem, Handle<Image>>,
}

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GenerateSpellsPlugin);
    }
}
