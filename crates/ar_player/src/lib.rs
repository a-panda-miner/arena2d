use ar_core::{
    AppState, Cooldown, CurrentStamina, Damage, Health, Layer, MagnetMarker, MaxHealth, MaxStamina,
    PlayerInvulnerableFrames, PlayerLastDirection, PlayerMarker, PlayerSet, StaminaRegen,
};
use ar_spells::generator::{OwnedProjectileSpells, ProjectileSpells};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InBattle),
            (spawn_player, setup_basic_spell).chain().in_set(PlayerSet),
        );
    }
}

// Once the player's entity is spawned, it should never be despawned
// so the player_id is always valid to deference
// TODO! Either initialize with a default and change it during the spawn_player() system,
// or make a exclusive &World system that handles initialization
#[derive(Resource)]
pub struct PlayerHandler {
    pub player_id: Entity,
}

#[derive(AssetCollection, Resource)]
pub struct SheetHandle {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 2, rows = 4))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "characters/character.png")]
    pub sprite: Handle<Image>,
}

fn spawn_player(mut commands: Commands, sheet_handle: Res<SheetHandle>) {
    let player_id = commands
        .spawn(PlayerMarker)
        .insert((
            SpriteBundle {
                texture: sheet_handle.sprite.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..Default::default()
            },
            TextureAtlas::from(sheet_handle.layout.clone()),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Mass(50.0))
        .insert(LinearVelocity(Vec2::ZERO))
        .insert(AngularVelocity(0.0))
        .insert(Collider::circle(5.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Cooldown(Timer::from_seconds(0.24, TimerMode::Repeating))) // Animation cooldown, for attacks it should be a Cooldown in its own child
        .insert(CollisionLayers::new(
            [Layer::Player],
            [Layer::Monster, Layer::MonsterProjectile],
        ))
        .insert(PlayerInvulnerableFrames {
            // Player invulnerability after getting hit
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .insert(Health(100))
        .insert(MaxHealth(100))
        .insert(CurrentStamina(1.))
        .insert(MaxStamina(10.))
        .insert(StaminaRegen(0.1))
        .insert(Damage(1))
        .insert(OwnedProjectileSpells { spells: vec![] })
        // This child is the magnet collider, used for item pickup
        .with_children(|children| {
            children.spawn((
                Collider::circle(1.0),
                CollisionLayers::new([Layer::Magnet], [Layer::Item]),
                MagnetMarker,
            ));
        })
        .id();

    commands.insert_resource(PlayerHandler { player_id });
    commands.insert_resource(PlayerLastDirection { direction: Vec2::Y });
}

// TODO! This should be chosen by the player at the menu before the game starts
// This function should be ran after the player is spawned and after the spells are set up
fn setup_basic_spell(
    mut player_spells: Query<&mut OwnedProjectileSpells, With<PlayerMarker>>,
    loaded_projectile_spells: Res<ProjectileSpells>,
) {
    let mut player_spells = player_spells.single_mut();
    let spell = loaded_projectile_spells
        .projectile_spells
        .get("throwdagger")
        .expect("no throwdagger in loaded spells");
    player_spells.spells.push(spell.clone());
}
