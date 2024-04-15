use ar_core::{BoostUsage, DashUsage, InputSet, PlayerDirection, ZoomOut, ZoomIn, PlayerMarker};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDirection>()
            .add_event::<BoostUsage>()
            .add_event::<DashUsage>()
            .add_systems(Update, (player_input_manager.in_set(InputSet), animate_player.in_set(InputSet)).chain());
    }
}

fn player_input_manager(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_direction: EventWriter<PlayerDirection>,
    mut ev_boost: EventWriter<BoostUsage>,
    mut ev_dash: EventWriter<DashUsage>,
    mut ev_zoom_out: EventWriter<ZoomOut>,
    mut ev_zoom_in: EventWriter<ZoomIn>,
) {
    let w = keys.pressed(KeyCode::KeyW);
    let a = keys.pressed(KeyCode::KeyA);
    let s = keys.pressed(KeyCode::KeyS);
    let d = keys.pressed(KeyCode::KeyD);
    let q = keys.pressed(KeyCode::KeyQ);
    let e = keys.pressed(KeyCode::KeyE);

    let boost = keys.pressed(KeyCode::ShiftLeft);
    let dash = keys.pressed(KeyCode::Space);

    let mut direction = Vec2::ZERO;

    if d {
        direction.x += 1.;
    }
    if a {
        direction.x += -1.;
    }
    if w {
        direction.y += 1.;
    }
    if s {
        direction.y += -1.;
    }
    // A normalized direction vector
    let direction = direction.normalize_or_zero();
    ev_direction.send(PlayerDirection(direction));
    if boost {
        ev_boost.send(BoostUsage(boost));
    }
    if dash {
        ev_dash.send(DashUsage(dash));
    }
    if q {
        ev_zoom_out.send(ZoomOut);
    }
    if e {
        ev_zoom_in.send(ZoomIn);
    }
}

fn animate_player(
    mut query: Query<&mut TextureAtlas, With<PlayerMarker>>,
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
        texture_atlas.index = 7;
        return;
    }
    if a {
        texture_atlas.index = 5;
        return;
    }
    if s {
        texture_atlas.index = 0;
        return;
    }
    if d {
        texture_atlas.index = 3;
        return;
    }
}