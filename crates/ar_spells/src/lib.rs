use ar_core::WeaponType;
use ar_enemies::MonsterLayoutType;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

#[derive(Debug, AssetCollection, Resource)]
pub struct SpellSprites {
    #[asset(paths("spells/dagger.png"), collection(mapped, typed))]
    pub spells_sheet: HashMap<AssetFileStem, Handle<Image>>,
}

#[derive(Clone, Deserialize, Debug)]
pub enum SpellType {
    Summon,
    Projectile,
    Swing,
    Buff,
    AoE,
}

#[derive(Clone, Deserialize, Debug)]
pub enum SpellAOEType {
    Arc,
    Circle,
    Rectangle,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellSummonType {
    pub summon_name: String,
    pub summon_sprite: String,
    pub summon_layout: Option<MonsterLayoutType>,
    pub summon_limit: u8,
    pub summon_hp: u32,
    pub summon_damage: u32,
    pub summon_movespeed: Option<f32>,
    pub summon_attackspeed: Option<f32>,
    pub summon_regen: Option<u32>,
    pub summon_attack_range: Option<u32>,
    pub summon_weapon: Option<WeaponType>,
    // If it is of type None then the summon is permanent
    pub summon_duration: Option<f32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellProjectileType {
    pub projectile_sprite: String,
    // If it is of type None then the projectile is invulnerable
    pub projectile_hp: Option<u32>,
    pub projectile_count: u8,
    pub projectile_pattern: ProjectilePattern,
    pub projectile_damage: u32,
    pub projectile_movespeed: Option<f32>,
    pub projectile_radius: f32, // The size of the collider
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellSwingType {
    pub swing_sprite: String,
    pub swing_damage: u32,
    pub swing_arc: f32,
    pub swing_length: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellBuffType {
    pub buff_duration: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub enum ProjectilePattern {
    Circle, // Shoots projectiles in a circle pattern, starting at 360° and reducing by 360°/n for each projectile, where n is projectile_count
    Line,   // Shoots projectiles one after the other in quick succession
}
