// Game's configuration
// Changes the behavior of the systems

use bevy::prelude::*;

pub const BG_COLOR: (u8, u8, u8) = (0, 0, 0);

pub const PFPS: f64 = 64.0;

/// Only used to insert the default resources in the game,
/// The resources themselves should be updated through the loading process,
/// through the menu or input
pub struct ConfPlugin;

impl Plugin for ConfPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConf::default());
    }
}

#[derive(Resource, Debug, Default)]
pub struct CameraConf {
    pub camera_mode: CameraMode,
}

#[derive(Debug, Default)]
pub enum CameraMode {
    Square,
    #[default]
    Follow,
}

#[derive(Debug, Default, Resource)]
pub struct NumbersDisplayConf {
    pub show_player_damaged_numbers: bool,
    pub show_player_dealt_damage_numbers: bool,
    pub show_player_healed_numbers: bool,
}

#[derive(Debug, Default, Resource)]
pub struct UiConf {
    pub show_ui: bool,
    pub show_fps: bool,
    pub ui_scale: f32,
}
