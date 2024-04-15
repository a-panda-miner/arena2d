use ar_core::{AppState, PlayerMarker, PlayerSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InBattle), spawn_player.in_set(PlayerSet));
        
    }
}

// Once the player's entity is spawned, it should never be despawned
// so the player_id is always valid to deference
// TODO! Either initialize with a default and change it during the spawn_player() system,
// or make a exclusive &World system that handles initialization
#[derive(Resource)]
struct PlayerHandler {
    player_id: Entity,
}

#[derive(AssetCollection, Resource)]
pub struct SheetHandle {
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 4))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "characters/character.png")]
    pub sprite: Handle<Image>,
}

fn spawn_player(mut commands: Commands, sheet_handle: Res<SheetHandle>) {
    commands
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
        .insert(LockedAxes::ROTATION_LOCKED);
}