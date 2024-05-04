// This crate is responsible for making the template of the
// enemies and handling their dynamic assets

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

#[derive(Debug, AssetCollection, Resource)]
pub struct MonsterSprites {
    #[asset(
        paths(
            "monsters/large/dummy.png",
            "monsters/small/slimesmall.png",
            "monsters/small/ratsmall.png",
            "monsters/small/zombiesmall.png",
            "monsters/small/rabbitsmall.png",
            "monsters/small/chickensmall.png",
            "monsters/small/batsmall.png"
        ),
        collection(mapped, typed)
    )]
    pub monster_sheets: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 1))]
    pub monster_layout_large_four: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 9, rows = 1))]
    pub monster_layout_large_nine: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 8., tile_size_y = 8., columns = 2, rows = 4))]
    pub monster_layout_small: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Deserialize, Debug)]
pub enum MonsterLayoutType {
    Small,
    Medium,
    Large,
    Boss,
}

#[derive(Clone, Deserialize, Debug)]
pub enum QualityMonster {
    Common,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Boss,
}

#[derive(Clone, Deserialize, Debug)]
pub enum MonsterAI {
    StateMachine,
    BehaviorTree,
    BigBrain,
}
