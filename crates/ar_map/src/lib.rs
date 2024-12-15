use ar_core::{AppState, MapSet, Layer};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tiled::prelude::*;
use avian2d::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_plugins(TiledMapPlugin::default())
            .add_systems(
                OnEnter(AppState::InBattle), 
                (spawn_map).in_set(MapSet),);
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");

    commands.spawn(TiledMapHandle(map_handle))
        .insert(TiledMapSettings {
            layer_positioning: LayerPositioning::Centered,
            ..Default::default()
        });

    // top wall
    commands
        .spawn(RigidBody::Static)
        .insert(Collider::rectangle(1960.0, 16.0))
        .insert(Mass(500.0))
        .insert(Transform::from_xyz(0.0, 640.0, 100.0)) 
        .insert(CollisionLayers::new(
            [Layer::Wall],
            [Layer::Player],
        ));

    // bottom wall
    commands
        .spawn(RigidBody::Static)
        .insert(Collider::rectangle(1960.0, 16.0))
        .insert(Mass(500.0))
        .insert(Transform::from_xyz(0.0, -624.0, 100.0)) 
        .insert(CollisionLayers::new(
            [Layer::Wall],
            [Layer::Player],
        ));

    // left wall
    commands
        .spawn(RigidBody::Static)
        .insert(Collider::rectangle(16.0, 1320.0))
        .insert(Mass(500.0))
        .insert(Transform::from_xyz(-944.0, 0.0, 100.0))
        .insert(CollisionLayers::new(
            [Layer::Wall],
            [Layer::Player],
        ));

    // right wall
    commands
        .spawn(RigidBody::Static)
        .insert(Collider::rectangle(16.0, 1320.0))
        .insert(Mass(500.0))
        .insert(Transform::from_xyz(960.0, 0.0, 100.0))
        .insert(CollisionLayers::new(
            [Layer::Wall],
            [Layer::Player],
        ));
}