use ar_core::{AppState, PlayerMarker, PlayerSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InBattle), spawn_player.in_set(PlayerSet))
            .add_systems(Update, animate_player.in_set(PlayerSet));
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
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "characters/player.png")]
    pub sprite: Handle<Image>,
}

fn spawn_player(mut commands: Commands, sheet_handle: Res<SheetHandle>) {
    commands
        .spawn(PlayerMarker)
        .insert((
            SpriteBundle {
                texture: sheet_handle.sprite.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

fn animate_player(
    mut query: Query<(&mut TextureAtlas), With<PlayerMarker>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut texture_atlas = query.single_mut();
    let w = keys.pressed(KeyCode::KeyW);
    let a = keys.pressed(KeyCode::KeyA);
    let s = keys.pressed(KeyCode::KeyS);
    let d = keys.pressed(KeyCode::KeyD);
    if !w && !a && !s && !d {
        return;
    }
    if w {
        texture_atlas.index = 0;
        return;
    }
    if a {
        texture_atlas.index = 3;
        return;
    }
    if s {
        texture_atlas.index = 2;
        return;
    }
    if d {
        texture_atlas.index = 1;
        return;
    }
}
