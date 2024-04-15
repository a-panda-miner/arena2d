// This crate is responsible for making the template of the
// enemies and handling their dynamic assets

use bevy::prelude::*;
use bevy::utils::HashMap;
use ar_core::{EnemiesSet, AppState};
use bevy_xpbd_2d::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Resource)]
pub struct MonsterMap {
    pub monster_templates: HashMap<String, MonsterTemplate>,
}


#[derive(Debug, AssetCollection, Resource)]
pub struct MediumMonsterSprites {
    #[asset(paths(
        "monsters/medium/enemy01.png"), collection(mapped, typed))]
    pub monster_sheets: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 5, rows = 1))]
    pub monster_layout: Handle<TextureAtlasLayout>,
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

pub enum MonsterLayoutType {
    Small,
    Medium,
    Large,
    Boss,
}

pub enum RewardType {
    Currency,
    SpecialCurrency,
    PetXp
}

pub enum QualityMonster {
    Common,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Boss,
}

pub struct EnemiesPlugin;

struct MonsterTemplate {
    name: String,
    base_health: u32,
    base_movespeed: f32,
    base_regen: Option<u32>,
    reward_type: RewardType,
    quality: QualityMonster,
    layout_size: MonsterLayoutType,
}

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InBattle), spawn_enemy_dummy.in_set(EnemiesSet));
    }
}

fn spawn_enemy_dummy(
    mut commands: Commands,
    monster_map: Res<MediumMonsterSprites>,
) {
    info!("monster map: {:?}", monster_map);
    commands.spawn(
        SpriteBundle {
            texture: monster_map.monster_sheets.get("enemy01").unwrap().clone(),
            transform: Transform::from_xyz(50.0, 0.0, 4.0),
            ..Default::default()
        }
    )
    .insert(TextureAtlas::from(monster_map.monster_layout.clone()))
    .insert(RigidBody::Dynamic)
    .insert(Mass(50.0))
    .insert(LinearVelocity(Vec2::ZERO))
    .insert(AngularVelocity(0.0))
    .insert(Collider::circle(5.0));
}

