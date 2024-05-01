// This crate is responsible for making the template of the
// enemies and handling their dynamic assets

use bevy::prelude::*;
use bevy::utils::HashMap;
use ar_core::{EnemiesSet, AppState};
use bevy_xpbd_2d::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

#[derive(Debug, AssetCollection, Resource)]
pub struct MonsterSprites {
    #[asset(paths(
        "monsters/large/dummy.png", "monsters/small/slimesmall.png", "monsters/small/ratsmall.png",
        "monsters/small/zombiesmall.png", "monsters/small/rabbitsmall.png", "monsters/small/chickensmall.png",
        "monsters/small/batsmall.png"), collection(mapped, typed))]
    pub monster_sheets: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 1))]
    pub monster_layout_large_four: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 9, rows = 1))]
    pub monster_layout_large_nine: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 8., tile_size_y = 8., columns = 2, rows = 4))]
    pub monster_layout_small: Handle<TextureAtlasLayout>,
}

/*
// Bugged, but uses the folder instead of full paths for sprites
#[derive(Debug, AssetCollection, Resource)]
pub struct MediumMonsterSprites {
    #[asset(path = "monsters/medium", collection(typed, mapped))]
    pub monster_sheets: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 5, rows = 1))]
    pub monster_layout: Handle<TextureAtlasLayout>,
}
*/

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

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(OnEnter(AppState::InBattle), spawn_enemy_dummy.in_set(EnemiesSet));
    }
}

/* 
fn spawn_enemy_dummy(
    mut commands: Commands,
    monster_sprites: Res<MonsterSprites>,
) {
    commands.spawn(
        SpriteBundle {
            texture: monster_sprites.monster_sheets.get("dummy").unwrap().clone(),
            transform: Transform::from_xyz(50.0, 0.0, 4.0),
            ..Default::default()
        }
    )
    .insert(TextureAtlas::from(monster_sprites.monster_layout_large_four.clone()))
    .insert(RigidBody::Dynamic)
    .insert(Mass(50.0))
    .insert(LinearVelocity(Vec2::ZERO))
    .insert(AngularVelocity(0.0))
    .insert(Collider::circle(5.0));
}
*/