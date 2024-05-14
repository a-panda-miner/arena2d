use ar_core::{
    BGMusicMarker, BoostUsage, ChangeBackgroundEvent, Cooldown, DashUsage, InputSet,
    PlayerDirection, PlayerLastDirection, PlayerMarker, ZoomIn, ZoomOut,
};
use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Resource)]
struct BButtonCooldown(Timer);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDirection>()
            .add_event::<BoostUsage>()
            .add_event::<DashUsage>()
            .add_event::<ChangeBackgroundEvent>()
            .insert_resource(BButtonCooldown(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_systems(
                Update,
                (
                    player_input_manager.in_set(InputSet),
                    animate_player.in_set(InputSet),
                    animate_player_loop.in_set(InputSet),
                    change_background_music.in_set(InputSet),
                )
                    .chain(),
            );
    }
}

fn player_input_manager(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_direction: EventWriter<PlayerDirection>,
    mut ev_boost: EventWriter<BoostUsage>,
    mut ev_dash: EventWriter<DashUsage>,
    mut ev_zoom_out: EventWriter<ZoomOut>,
    mut ev_zoom_in: EventWriter<ZoomIn>,
    mut last_direction: ResMut<PlayerLastDirection>,
) {
    let w = keys.pressed(KeyCode::KeyW);
    let a = keys.pressed(KeyCode::KeyA);
    let s = keys.pressed(KeyCode::KeyS);
    let d = keys.pressed(KeyCode::KeyD);
    let q = keys.pressed(KeyCode::KeyQ);
    let e = keys.pressed(KeyCode::KeyE);

    let boost = keys.pressed(KeyCode::ShiftLeft);
    let dash = keys.pressed(KeyCode::Space);

    if !w && !a && !s && !d && !q && !e && !boost && !dash {
        return;
    }
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
    last_direction.direction = direction;
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

fn animate_player_loop(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut Cooldown), With<PlayerMarker>>,
) {
    let (mut texture_atlas, mut cooldown) = query.single_mut();
    if cooldown.0.tick(time.delta()).just_finished() {
        match texture_atlas.index {
            0 => texture_atlas.index = 1,
            1 => texture_atlas.index = 0,
            2 => texture_atlas.index = 3,
            3 => texture_atlas.index = 2,
            4 => texture_atlas.index = 5,
            5 => texture_atlas.index = 4,
            6 => texture_atlas.index = 7,
            7 => texture_atlas.index = 6,
            _ => {}
        }
    }
}

fn change_background_music(
    time: Res<Time>,
    mut timer: ResMut<BButtonCooldown>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev: EventWriter<ChangeBackgroundEvent>,
    mut query: Query<&mut Cooldown, With<BGMusicMarker>>,
) {
    let mut flag = false;
    for mut cooldown in query.iter_mut() {
        if cooldown.0.tick(time.delta()).just_finished() {
            flag = true;
        }
    }
    let a = timer.0.tick(time.delta()).finished();
    let mut b = keys.pressed(KeyCode::KeyB);
    if !a {
        b = false;
    }
    if !b && !flag {
        return;
    }
    ev.send(ChangeBackgroundEvent);
    timer.0.reset();
}
