use crate::*;

pub struct GenerateSpellsPlugin;

impl Plugin for GenerateSpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InBattle),
            setup_generate_spells.in_set(SpellSet),
        );
    }
}
/*
/// A hashmap of swingspells that exist
#[derive(Resource)]
pub struct SwingSpells {
    swing_spells: HashMap<String, SpellSwing>,
}
*/
/// A hashmap of projectilespells that exist
#[derive(Resource, Debug)]
pub struct ProjectileSpells {
    pub projectile_spells: HashMap<String, SpellProjectile>,
}

/// The spells of the type Projectile that the Entity has
#[derive(Component)]
pub struct OwnedProjectileSpells {
    pub spells: Vec<SpellProjectile>,
}

/// The spells of the type Swing that the Entity has
#[derive(Component)]
pub struct OwnedSwingSpells {
    pub spells: Vec<SpellSwing>,
}

#[derive(Clone, Debug)]
pub struct SpellProjectile {
    pub name: String,
    pub sprite: String,
    pub cooldown: Timer,
    pub count: u8,
    pub pattern: ProjectilePattern,
    pub damage: usize,
    pub projectile_movespeed: f32,
    pub radius: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub penetration: u8,
}

#[derive(Clone, Debug)]
pub struct SpellProjectileExplosive {
    pub name: String,
    pub sprite: String,
    pub cooldown: Timer,
    pub count: u8,
    pub pattern: ProjectilePattern,
    pub projectile_radius: f32,
    pub projectile_mass: f32,
    pub projectile_movespeed: f32,
    pub projectile_lifetime: f32,
    pub damage_max: usize,
    pub damage_min: usize,
    pub explosion_radius: f32,
    pub max_damage_aoe: f32,
    pub min_damage_aoe: f32,
    pub trail_damage: Option<usize>,
    pub trail_lifetime: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct SpellSwing {
    pub name: String,
    pub sprite: String,
    pub cooldown: f32,
    pub damage: u32,
    pub arc: f32,
    pub length: f32,
    pub aoe: SpellAOEType,
}

/// Creates the SpellSwing, SpellProjectile structs from the SpellTemplates
/// Must be run before setup_player as the player is spawned with a spell
pub fn setup_generate_spells(loaded_spells: Res<SpellTemplates>, mut commands: Commands) {
    let mut projectile_spells = HashMap::new();
    for (name, spell) in &loaded_spells.spells {
        if let SpellType::Projectile = spell.spell_main_type {
            let projectile = spell
                .projectile_type_struct
                .clone()
                .expect("Projectile Type with no Projectile Struct");
            let proj = SpellProjectile {
                name: spell.name.clone(),
                sprite: projectile.projectile_sprite,
                cooldown: Timer::from_seconds(spell.cooldown, TimerMode::Repeating),
                count: projectile.projectile_count,
                pattern: projectile.projectile_pattern,
                damage: projectile.projectile_damage,
                projectile_movespeed: projectile.projectile_movespeed,
                radius: projectile.projectile_radius,
                mass: projectile.projectile_mass,
                lifetime: projectile.projectile_lifetime,
                penetration: projectile.projectile_penetration.unwrap_or(0),
            };
            projectile_spells.insert(name.clone(), proj);
        }
    }
    commands.insert_resource(ProjectileSpells { projectile_spells });
}
