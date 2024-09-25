#[cfg(debug_assertions)]
use ar_core::OneShotSystems;
use ar_core::{
    BoostUsage, CameraFollowState, ChangeBackgroundEvent, DashUsage, InputSet, PlayerDirection,
    PlayerMarker, ZoomIn, ZoomOut,
};

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_event::<PlayerDirection>()
            .add_event::<BoostUsage>()
            .add_event::<DashUsage>()
            .add_event::<ChangeBackgroundEvent>()
            .init_resource::<ActionState<Action>>()
            .insert_resource(Action::input_map())
            .add_systems(
                Update,
                (
                    player_movement_direction.in_set(InputSet),
                    dash.in_set(InputSet),
                    boost.in_set(InputSet),
                    change_background_music.in_set(InputSet),
                    change_camera_follow_state.in_set(InputSet),
                    zoom_in_out.in_set(InputSet),
                    player_animation
                        .run_if(on_timer(Duration::from_millis(240)))
                        .in_set(InputSet),
                )
                    .chain(),
            );
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (check_input, give_exp_debug)
                .in_set(InputSet)
                .before(player_movement_direction),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    ZoomIn,
    ZoomOut,
    ChangeMusic,
    ChangeCamera,
    Dash,
    Boost,
    ChooseCard1,
    ChooseCard2,
    ChooseCard3,
    #[cfg(debug_assertions)]
    GiveExpDebug,
}

impl Action {
    const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn direction(self) -> Option<Dir2> {
        match self {
            Self::Up => Some(Dir2::Y),
            Self::Down => Some(Dir2::NEG_Y),
            Self::Left => Some(Dir2::NEG_X),
            Self::Right => Some(Dir2::X),
            _ => None,
        }
    }

    fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Up, KeyCode::KeyW);
        input_map.insert(Self::Down, KeyCode::KeyS);
        input_map.insert(Self::Left, KeyCode::KeyA);
        input_map.insert(Self::Right, KeyCode::KeyD);
        input_map.insert(Self::ChangeMusic, KeyCode::KeyV);
        input_map.insert(Self::ChangeCamera, KeyCode::KeyC);
        input_map.insert(Self::Dash, KeyCode::Space);
        input_map.insert(Self::Boost, KeyCode::ShiftLeft);
        input_map.insert(Self::ZoomIn, KeyCode::KeyQ);
        input_map.insert(Self::ZoomOut, KeyCode::KeyE);
        input_map.insert(Self::ChooseCard1, KeyCode::Digit1);
        input_map.insert(Self::ChooseCard2, KeyCode::Digit2);
        input_map.insert(Self::ChooseCard3, KeyCode::Digit3);
        #[cfg(debug_assertions)]
        input_map.insert(Self::GiveExpDebug, KeyCode::KeyM);

        input_map
    }
}

fn player_movement_direction(
    action_state: Res<ActionState<Action>>,
    mut ev_direction: EventWriter<PlayerDirection>,
) {
    let mut direction_vec = Vec2::ZERO;

    for input_direction in Action::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vec += *direction;
            }
        }
    }

    let direction = direction_vec.normalize_or_zero();
    if direction != Vec2::ZERO {
        ev_direction.send(PlayerDirection(direction));
    }
}

fn dash(action_state: Res<ActionState<Action>>, mut ev_dash: EventWriter<DashUsage>) {
    if action_state.just_pressed(&Action::Dash) {
        ev_dash.send(DashUsage(true));
    }
}

fn boost(action_state: Res<ActionState<Action>>, mut ev_boost: EventWriter<BoostUsage>) {
    if action_state.pressed(&Action::Boost) {
        ev_boost.send(BoostUsage(true));
    }
}

pub fn change_background_music(
    action_state: Res<ActionState<Action>>,
    mut ev: EventWriter<ChangeBackgroundEvent>,
) {
    if action_state.just_pressed(&Action::ChangeMusic) {
        ev.send(ChangeBackgroundEvent);
    }
}

pub fn change_camera_follow_state(
    action_state: Res<ActionState<Action>>,
    mut camera_state: ResMut<CameraFollowState>,
) {
    if action_state.just_pressed(&Action::ChangeCamera) {
        let state = match *camera_state {
            CameraFollowState::Player => CameraFollowState::Rect,
            CameraFollowState::Rect => CameraFollowState::Player,
        };
        *camera_state = state;
    }
}

pub fn zoom_in_out(
    action_state: Res<ActionState<Action>>,
    mut ev_zoom_in: EventWriter<ZoomIn>,
    mut ev_zoom_out: EventWriter<ZoomOut>,
) {
    if action_state.just_pressed(&Action::ZoomIn) {
        ev_zoom_in.send(ZoomIn);
    }
    if action_state.just_pressed(&Action::ZoomOut) {
        ev_zoom_out.send(ZoomOut);
    }
}

fn player_animation(
    action_state: Res<ActionState<Action>>,
    mut query: Query<&mut TextureAtlas, With<PlayerMarker>>,
) {
    let mut texture_atlas = query.get_single_mut().unwrap();

    if action_state.pressed(&Action::Up) && action_state.pressed(&Action::Right) {
        if texture_atlas.index == 8 {
            texture_atlas.index = 9;
        } else {
            texture_atlas.index = 8;
        }
        return;
    }
    if action_state.pressed(&Action::Up) && action_state.pressed(&Action::Left) {
        if texture_atlas.index == 10 {
            texture_atlas.index = 11;
        } else {
            texture_atlas.index = 10;
        }
        return;
    }
    if action_state.pressed(&Action::Down) && action_state.pressed(&Action::Left) {
        if texture_atlas.index == 12 {
            texture_atlas.index = 13;
        } else {
            texture_atlas.index = 12;
        }
        return;
    }
    if action_state.pressed(&Action::Down) && action_state.pressed(&Action::Right) {
        if texture_atlas.index == 14 {
            texture_atlas.index = 15;
        } else {
            texture_atlas.index = 14;
        }
        return;
    }
    if action_state.pressed(&Action::Down) {
        if texture_atlas.index == 0 {
            texture_atlas.index = 1;
        } else {
            texture_atlas.index = 0;
        }
        return;
    }
    if action_state.pressed(&Action::Right) {
        if texture_atlas.index == 2 {
            texture_atlas.index = 3;
        } else {
            texture_atlas.index = 2;
        }
        return;
    }
    if action_state.pressed(&Action::Left) {
        if texture_atlas.index == 4 {
            texture_atlas.index = 5;
        } else {
            texture_atlas.index = 4;
        }
        return;
    }
    if action_state.pressed(&Action::Up) {
        if texture_atlas.index == 6 {
            texture_atlas.index = 7;
        } else {
            texture_atlas.index = 6;
        }
    }
}

#[cfg(debug_assertions)]
pub fn check_input(action_state: Res<ActionState<Action>>) {
    for action in action_state.get_pressed() {
        info!("Pressed {action:?}");
    }
}

#[cfg(debug_assertions)]
pub fn give_exp_debug(
    mut commands: Commands,
    one_shot: Res<OneShotSystems>,
    action_state: Res<ActionState<Action>>,
) {
    if action_state.just_pressed(&Action::GiveExpDebug) {
        let id = one_shot.0.get("give_exp").expect("no give_exp system");
        commands.run_system(*id);
    }
}
