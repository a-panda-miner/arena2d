use ar_core::{AppState, CameraSet, PlayerMarker, ZoomIn, ZoomOut};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Resource)]
struct CameraZoomState(u8);

#[derive(Component)]
struct ArenaCameraMarker;

impl Default for CameraZoomState {
    fn default() -> Self {
        CameraZoomState(1)
    }
}

const ARENA_OFFSET: f32 = 40.0;
const ARENA_HEIGHT_ZOOMOUT: f32 = 960.0;
const ARENA_HEIGHT_DEFAULT: f32 = 640.0;
const ARENA_HEIGHT_1_ZOOM: f32 = 480.0;
const ARENA_HEIGHT_2_ZOOM: f32 = 320.0;

const ARENA_WIDTH_ZOOMOUT: f32 = 480.0;
const ARENA_WIDTH_DEFAULT: f32 = 360.0;
const ARENA_WIDTH_1_ZOOM: f32 = 240.0;
const ARENA_WIDTH_2_ZOOM: f32 = 160.0;

pub struct ArenaCameraPlugin;

impl Plugin for ArenaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraZoomState>()
            .add_event::<ZoomIn>()
            .add_event::<ZoomOut>()
            .add_systems(
                OnEnter(AppState::InBattle),
                setup_arena_camera.in_set(CameraSet),
            )
            .add_systems(Update, 
                (change_camera_zoom, change_camera_state ,follow_player).in_set(CameraSet));
    }
}

fn setup_arena_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: (ARENA_WIDTH_DEFAULT),
        max_height: (ARENA_HEIGHT_DEFAULT),
    };
    // Allows us to always show ARENA_WIDTH and ARENA_HEIGHT of world units for the player in this cam
    commands.spawn((camera, ArenaCameraMarker));
}

fn change_camera_state(
    mut camera_state: ResMut<CameraZoomState>,
    mut zoom_in_event: EventReader<ZoomIn>,
    mut zoom_out_event: EventReader<ZoomOut>,
    mut last_change: Local<usize>,
) {
    *last_change += 1;
    if *last_change < 5 || (zoom_in_event.is_empty() && zoom_out_event.is_empty()) {
        return ()
    }
    if !zoom_in_event.is_empty() {
        match camera_state.0 {
            0 => camera_state.0 = 1,
            1 => camera_state.0 = 2,
            2 => camera_state.0 = 3,
            _ => {}
        }
    }
    if !zoom_out_event.is_empty() {
        match camera_state.0 {
            3 => camera_state.0 = 2,
            2 => camera_state.0 = 1,
            1 => camera_state.0 = 0,
            _ => {}
        }
    }
    zoom_in_event.clear();
    zoom_out_event.clear();
    //info!("Current zoom level: {}", camera_state.0);
    *last_change = 0;
}


// Changes the camera when the resource CameraZoomState changes
fn change_camera_zoom(
    camera_state: ResMut<CameraZoomState>,
    mut query: Query<&mut OrthographicProjection, With<ArenaCameraMarker>>,
) {
    if camera_state.is_changed() {
        let mut camera = query.single_mut();
        let (new_resolution_width, new_resolution_height) = map_zoom_level_to_scale(camera_state.0);
        camera.scaling_mode = ScalingMode::AutoMax {
            max_width: new_resolution_width,
            max_height: new_resolution_height,
        }
    } else {
        return;
    }
}

fn map_zoom_level_to_scale(zoom_level: u8) -> (f32, f32) {
    match zoom_level {
        0 => (ARENA_HEIGHT_ZOOMOUT, ARENA_WIDTH_ZOOMOUT),
        1 => (ARENA_HEIGHT_DEFAULT, ARENA_WIDTH_DEFAULT),
        2 => (ARENA_HEIGHT_1_ZOOM, ARENA_WIDTH_1_ZOOM),
        3 => (ARENA_HEIGHT_2_ZOOM, ARENA_WIDTH_2_ZOOM),
        _ => (ARENA_HEIGHT_DEFAULT, ARENA_WIDTH_DEFAULT),
    }
}

// TODO! Make the camera a physical object and follow the player with Bevy XPBD Interp 
fn follow_player(
    player_query: Query<&Transform, (With<PlayerMarker>, Without<ArenaCameraMarker>)>,
    mut camera_query: Query<&mut Transform, (With<ArenaCameraMarker>, Without<PlayerMarker>)>,
    camera_zoom: Res<CameraZoomState>,
) {
    let player_transform = player_query.single();
    let mut camera = camera_query.single_mut();
    let camera_x = camera.translation.x;
    let camera_y = camera.translation.y;
    let (height, width) = map_zoom_level_to_scale(camera_zoom.0);
    if (player_transform.translation.x - camera_x).abs() > (height / 3.5) {
        camera.translation.x = player_transform.translation.x;
    }
    if (player_transform.translation.y - camera_y).abs() > (width / 3.5) {
        camera.translation.y = player_transform.translation.y;
    }
}