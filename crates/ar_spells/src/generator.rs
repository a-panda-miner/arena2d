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

/// A hashmap of swingspells that exist
#[derive(Resource)]
pub struct SwingSpells {
    swing_spells: HashMap<String, SpellSwing>,
}

/// A hashmap of projectilespells that exist
#[derive(Resource)]
pub struct ProjectileSpells {
    projectile_spells: HashMap<String, SpellProjectile>,
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

pub struct SpellProjectile {
    name: String,
    sprite: String,
    cooldown: f32,
    count: u8,
    pattern: ProjectilePattern,
    damage: u32,
    movespeed: Option<f32>,
    radius: f32,
}

pub struct SpellSwing {
    name: String,
    sprite: String,
    cooldown: f32,
    damage: u32,
    arc: f32,
    length: f32,
    aoe: SpellAOEType,
}

/// Creates the SpellSwing, SpellProjectile structs from the SpellTemplates
/// Must be run before setup_player as the player is spawned with a spell
pub fn setup_generate_spells(loaded_spells: Res<SpellTemplates>, mut commands: Commands) {
    let mut projectile_spells = HashMap::new();
    for (_, &ref spell) in &loaded_spells.spells {
        match spell.spell_main_type {
            SpellType::Projectile => {
                let projectile = spell
                    .projectile_type_struct
                    .clone()
                    .expect("Projectile Type with no Projectile Struct");
                let proj = SpellProjectile {
                    name: spell.name.clone(),
                    sprite: projectile.projectile_sprite,
                    cooldown: spell.cooldown,
                    count: projectile.projectile_count,
                    pattern: projectile.projectile_pattern,
                    damage: projectile.projectile_damage,
                    movespeed: projectile.projectile_movespeed,
                    radius: projectile.projectile_radius,
                };
                projectile_spells.insert(proj.name.clone(), proj);
            }
            _ => (),
        }
    }
    commands.insert_resource(ProjectileSpells { projectile_spells });
}
