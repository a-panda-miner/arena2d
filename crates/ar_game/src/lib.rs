use ar_battle::BattlePlugin;
use ar_camera::ArenaCameraPlugin;
use ar_conf::{BG_COLOR, PFPS};
use ar_core::{AppState, CameraSet, PlayerSet, InputSet, MapSet};
use ar_input::InputPlugin;
use ar_player::{PlayerPlugin, SheetHandle};
use ar_enemies::{EnemiesPlugin, MediumMonsterSprites};
use ar_map::{MapPlugin, TilesetHandle};
use ar_audio::{GameAudioPlugin, GameAudioAssets, BackGroundMusic};

use bevy::{
    core::TaskPoolThreadAssignmentPolicy,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_asset_loader::{prelude::*};
use bevy_xpbd_2d::prelude::*;
use iyes_progress::{ProgressPlugin};
use bevy_fast_tilemap::{FastTileMapPlugin};

use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(
                DefaultPlugins
                    .set(LogPlugin {
                        level: Level::INFO,
                        filter: "wpgu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                        update_subscriber: None,
                    })
                    .set(TaskPoolPlugin {
                        task_pool_options: TaskPoolOptions {
                            min_total_threads: 1,
                            max_total_threads: std::usize::MAX,
                            io: TaskPoolThreadAssignmentPolicy {
                                min_threads: 1,
                                max_threads: 1,
                                percent: 10.0,
                            },
                            async_compute: TaskPoolThreadAssignmentPolicy {
                                min_threads: 1,
                                max_threads: 1,
                                percent: 10.0,
                            },
                            compute: TaskPoolThreadAssignmentPolicy {
                                min_threads: 1,
                                max_threads: std::usize::MAX,
                                percent: 50.0,
                            },
                        },
                    })
                    .set(ImagePlugin::default_nearest())
                    .build(),
            )
            .add_plugins(
                ProgressPlugin::new(AppState::LoadingAssets).continue_to(AppState::InBattle),
            )
            .add_plugins(FastTileMapPlugin::default())
            .add_plugins(ArenaCameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemiesPlugin)
            .add_plugins(BattlePlugin)
            .add_plugins(MapPlugin)
            .add_plugins(GameAudioPlugin)
            .add_plugins(PhysicsPlugins::new(FixedUpdate))
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            .insert_resource(Time::<Fixed>::from_hz(PFPS))
            .insert_resource(BackGroundMusic::default())
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::InBattle)
                    .load_collection::<MediumMonsterSprites>()
                    .load_collection::<SheetHandle>()
                    .load_collection::<TilesetHandle>()
                    .load_collection::<GameAudioAssets>(),
            )
            .insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::rgba_u8(
                BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
            )))
            .insert_resource(Gravity(Vec2::ZERO))
            .configure_sets(Update, (
                CameraSet.run_if(in_state(AppState::InBattle)),
                PlayerSet.run_if(in_state(AppState::InBattle)),
                InputSet.run_if(in_state(AppState::InBattle)),
                MapSet.run_if(in_state(AppState::InBattle)),
            ));
    }
}
