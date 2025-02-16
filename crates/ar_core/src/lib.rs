// Structs and enums that are used by more than one crate
// to avoid a circular dependency

use avian2d::prelude::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

pub trait FromReader<R> {
    fn from_reader(reader: R) -> Result<Self, ron::error::SpannedError>
    where
        Self: Sized;
}

/// Defines the maximum level in a run
pub const MAX_LEVEL: u8 = 200;

#[derive(Resource, Debug)]
pub struct LevelTable {
    pub table: [usize; MAX_LEVEL as usize],
}

/// Defines tha main states of the app
/// LoadingAssets loads the assets during the App startup,
/// LoadingTemplates initiates some of those assets into resources,
/// Setup reads the save file and loads the associated resources and options (not implemented)
/// InitialScreen is the main menu before starting the game (not implemented)
/// InBattle is the main game state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    LoadingTemplates,
    Setup,
    InitialScreen,
    InBattle,
}

/// Different states of pause,
/// Paused is a manual pause of the game, (not implemented)
/// Menu is in the pause menu (not implemented)
/// Shop is triggered by a shop NPC (not implemented)
/// PowerUp is the selection of a power up after reaching a new level (not implemented)
/// MetaUpgrades is the buying of meta upgrades after dying (not implemented)
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PauseState {
    #[default]
    NotPaused,
    Paused,
    Menu,
    Shop,
    PowerUp,
    MetaUpgrades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ItemType {
    Coin,
    Ore,
    Lumber,
    Diamond,
    ExperienceOrb,
    Booster,
}

#[derive(Component, Debug)]
pub struct ItemComponent {
    pub item_type: ItemType,
    pub value: usize,
}

/// These systems are One-Shot systems, they are ran by calling commands.run_system(id)
#[derive(Resource, Debug)]
pub struct OneShotSystems(pub HashMap<String, SystemId>);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BattleSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemiesSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AISet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingTemplatesSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MonsterSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UtilSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpellSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardSet;

/// The base speed of the entity,
/// Needed for reference when applying and removing slow/stun effects
#[derive(Component)]
pub struct BaseSpeed(pub f32);

#[derive(Component)]
pub struct Stunned;

#[derive(Component)]
pub struct Silenced;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct MonsterMarker;

#[derive(Component)]
pub struct ItemMarker;

/// The marker for monsters with small layout,
/// used for animation purposes
#[derive(Component)]
pub struct MonsterMarkerSmall;

/// A hashset containing entities' ID that the projectile has collided with already,
/// so there are no collisions events happening multiple times when a projectile collides
/// with a monster/object
/// Performance concerns:
/// to avoid extra allocations initiate with 'with_capacity(n)', where n is the projectile_penetration number
#[derive(Component)]
pub struct CollidedHash(pub HashSet<Entity>);

/// The number of times the projectile can hit different monsters before despawning
#[derive(Component)]
pub struct Penetration(pub u8);

#[derive(Component)]
pub struct UiMarker;

/// Armor reduces damage taken
#[derive(Component)]
pub struct Armor;

/// Shield prevents damage
#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct Cooldown(pub Timer);

/// The timer must be set on Once, and then reset when the player takes damage or
/// is in an invulnerable state, when the timer finishes the player can take damage
#[derive(Component)]
pub struct PlayerInvulnerableFrames {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct MaxHealth(pub usize);

#[derive(Component)]
pub struct Damage(pub usize);

/// The marker for the background music
#[derive(Component)]
pub struct BGMusicMarker;

/// An event that is triggered when the player is hit,
/// not used yet for player's death
#[derive(Event)]
pub struct PlayerMinusHpEvent {
    pub damage: usize,
}

/// The direction in which the player will move
#[derive(Debug, Event)]
pub struct PlayerDirection(pub Vec2);

/// Boosts the movespeed of the player
#[derive(Debug, Event)]
pub struct BoostUsage(pub bool);

/// Used for moving the player in a dash event,
/// not implemented
#[derive(Debug, Event)]
pub struct DashUsage(pub bool);

/// Zooms in the camera,
/// the camera zoom levels is predetermined in the camera plugin
#[derive(Debug, Event)]
pub struct ZoomIn;

/// Zooms out the camera,
/// the camera zoom levels is predetermined in the camera plugin
#[derive(Debug, Event)]
pub struct ZoomOut;

#[derive(Debug, Event)]
pub struct DropItemEvent {
    pub position: Vec3,
    pub loot_table: LootTable,
}

/// An event to spawn items
#[derive(Debug, Event)]
pub struct SpawnItemEvent {
    pub name: String,
    pub value_added: usize,
    pub position: Vec3,
}

/// An event that is triggered when the player picks up an item
#[derive(Debug, Event)]
pub struct PickupEvent {
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct LevelUpEvent {
    pub level: u8,
}

#[derive(Debug, Event)]
pub struct ChosenCard(pub u8);

#[derive(Debug, Event)]
pub struct ApplyCard {
    pub card: String,
}

/// Changes the camera following strategy of the game
/// Rect is the default, adjusts itself only when the player moves out of the current 'rect'
/// Player moves the camera to follow the player each frame
#[derive(Resource, Default, Debug, PartialEq)]
pub enum CameraFollowState {
    #[default]
    Rect,
    Player,
}

/// Changes the background music of the game
#[derive(Debug, Event)]
pub struct ChangeBackgroundEvent;

/// An event that is triggered when the target reaches 0 HP,
/// despawning it and applying the death animation and rewards to
/// the player.
/// The player's death is not handled by this event
#[derive(Debug, Event)]
pub struct DeathEvent {
    pub target: Entity,
}

/// Displays the damage that happened on the ground,
/// damage done to the player is not handled by this event
#[derive(Debug, Event)]
pub struct DisplayDamageEvent {
    pub damage: usize,
    pub target: Entity,
}

#[derive(Clone, Deserialize, Debug)]
pub enum MonsterLayoutType {
    Small,
    Medium,
    Large,
    Boss,
}

#[derive(Clone, Deserialize, Debug)]
pub enum SpellType {
    Summon,
    Projectile,
    ProjectileExplosive,
    Swing,
    Buff,
    AoE,
}

/// Reward is automatically given when a condition is met
#[derive(Clone, Debug, Deserialize)]
pub enum RewardType {
    Currency,
    SpecialCurrency,
    PetXp,
}

/// Loot table
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct LootTable(pub u8);

impl From<u8> for LootTable {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Component, Deserialize)]
pub struct LootTables(pub Vec<LootTable>);

impl From<Vec<u8>> for LootTables {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into_iter().map(LootTable::from).collect())
    }
}

#[derive(Clone, Debug, Component, Deserialize)]
pub struct DropsChance(pub f32);

impl From<f32> for DropsChance {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Component, Deserialize)]
pub enum WeaponType {
    Dagger,
}

#[derive(Debug, Component)]
pub struct LifeTime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SummonSpellMarker;

#[derive(Component)]
pub struct AoESpellMarker;

#[derive(Component)]
pub struct ProjectileSpellMarker;

#[derive(Component)]
pub struct SwingSpellMarker;

#[derive(Component)]
pub struct BuffSpellMarker;

#[derive(Component)]
pub struct PlayerProjectileMarker;

#[derive(Component)]
pub struct MonsterProjectileMarker;

#[derive(Component)]
pub struct MagnetMarker;

#[derive(PhysicsLayer, Default)]
pub enum Layer {
    Player,
    Monster,
    #[default]
    Neutral,
    PlayerProjectile,
    PlayerAOE,
    MonsterProjectile,
    Pet,
    Item,
    Magnet,
    Wall,
}

#[derive(Resource, Debug)]
pub struct PickupRadius {
    pub default_radius: f32,
    pub multiplier: f32,
    pub max_radius: f32,
}

/// Keeps the internal score of the game,
/// used for spawning rules
#[derive(Resource)]
pub struct GameScore(pub usize);

/// Used for calculating GameScore
/// and spawning monsters
#[derive(Resource)]
pub struct MonstersAlive(pub usize);

/// Used for calculating GameScore
/// and spawning bosses
#[derive(Resource)]
pub struct MinutesSurvived(pub usize);

#[derive(Component)]
pub struct PlayerExperience(pub usize);

/// The level of the player
/// The level starts at 1
#[derive(Component)]
pub struct PlayerLevel(pub u8);

#[derive(Resource)]
pub struct AvailableCards(pub u8);

/// Used for spawning projectiles and dashing from where the player was last facing
#[derive(Resource)]
pub struct PlayerLastDirection {
    pub direction: Vec2,
}

/// Defines the cooldown of the dash and if the player is dashing
#[derive(Resource)]
pub struct PlayerDash {
    pub timer: Timer,
    pub cooldown_reduction: f32, // 0.0 - 1.0
    pub dashing: bool,
}

/// The max stamina the entity can have
#[derive(Component)]
pub struct MaxStamina(pub f32);

/// The current stamina of the entity
#[derive(Component)]
pub struct CurrentStamina(pub f32);

/// Regenerates stamina over time
#[derive(Component)]
pub struct StaminaRegen(pub f32);

#[derive(Clone, Deserialize, Debug)]
pub enum SpellAOEType {
    Arc(f32),
    Circle,
    Rectangle,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellAOE {
    pub aoe_sprite: String,
    pub aoe_radius: f32,
    pub aoe_damage: usize,
    pub aoe_pattern: SpellAOEType,
    pub aoe_knockback: Option<f32>,
    pub aoe_distributed: bool, // whether the total damage is distributed among the targets
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
    pub projectile_damage: usize,
    pub projectile_movespeed: f32,
    pub projectile_radius: f32, // The size of the collider
    pub projectile_mass: f32,   // The mass of the collider
    pub projectile_lifetime: f32,
    pub projectile_bounce: Option<bool>,
    pub projectile_penetration: Option<u8>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpellProjectileExplosiveType {
    pub projectile_sprite: String,
    pub projectile_radius: f32,
    pub projectile_mass: f32,
    pub projectile_movespeed: f32,
    pub projectile_lifetime: f32,
    pub damage_max: usize,
    pub damage_min: usize,
    pub explosion_sprite: String,
    pub max_damage_aoe: f32,
    pub min_damage_aoe: f32,
    pub trail_damage: Option<usize>,
    pub trail_lifetime: Option<f32>,
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

// Once the player's entity is spawned, it should never be despawned
// so the player_id is always valid to deference
// TODO! Either initialize with a default and change it during the spawn_player() system,
// or make a exclusive &World system that handles initialization
#[derive(Resource)]
pub struct PlayerHandler {
    pub player_id: Entity,
}

/// Used to help collision logic, every collision that has this ID is
/// an item pickup event
#[derive(Resource)]
pub struct MagnetHandler {
    pub magnet_id: Entity,
}

#[derive(Deserialize, Debug, Component, Clone, PartialEq, Hash)]
pub enum CardType {
    Buff,
    Spell,
}

#[derive(Deserialize, Debug, Component, Clone, PartialEq, Hash)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    Mythical,
    Legendary,
    Ultimate,
}

#[derive(Deserialize, Debug, Component, Clone)]
pub enum PowerUp {
    HealthUp(u8),
    AttackUp(u8),
    ShieldUp(u8),
    SpeedUp(u8),
    LootUp(u8),
    DamageUp(u8),
    ExpUp(u8),
    StaminaUp(u8),
}

#[derive(Deserialize, Debug, Component, Clone)]
pub enum PermanentDebuff {
    HealthDown(u8),
    AttackDown(u8),
    StaminaDown(u8),
    ExpDown(u8),
    SpeedDown(u8),
}

/// Every time the player levels up, spawn 3 new cards,
/// every level is an index of the array, the player chooses always from the index 0 and then the vec is shifted,
/// If there are no valid remaining cards it will be a None in the array,
/// TODO! if the vec[0] is [None, None, None] grants the player resource and consume the array.
/// If the vec is empty it means the player can't currently choose a new card.
#[derive(Resource, Debug, Default)]
pub struct ChooseACard {
    pub cards: Vec<[Option<String>; 3]>,
}

/// When a card reaches max level, remove it from the list
#[derive(Resource)]
pub struct RemainingCardsByType {
    pub powerup_cards: Vec<String>,
    pub spell_cards: Vec<String>,
}


// Note: Spell cards need to verify if the spell exists
#[derive(Clone, Deserialize, Debug)]
pub struct CardsTemplate {
    pub name: String,
    pub card_type: CardType, // A card can be a power-up or a spell
    pub max_level: u8,
    pub sprite: String,
    pub rarity: CardRarity,
    pub description: String,
    pub upgrade: Option<PowerUp>,
    pub spell: Option<String>,
    pub debuff: Option<PermanentDebuff>,
    pub max_level_bonus: Option<PowerUp>,
}

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct CardsTemplates {
    pub cards: HashMap<String, CardsTemplate>,
}

impl FromReader<File> for CardsTemplates {
    fn from_reader(reader: File) -> Result<Self, ron::error::SpannedError> {
        from_reader(reader)
    }
}

/// A structure that keeps track of the current cards
/// the player has chosen, and their level,
/// needs to be initialized in the template loading stage, after the cards are loaded
#[derive(Resource, Debug)]
pub struct CurrentCards {
    pub cards: HashMap<String, u8>,
}
