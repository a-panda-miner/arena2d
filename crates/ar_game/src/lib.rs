// This crate creates the GamePlugin, which is the only Plugin ran in the app

use ar_audio::{GameAudioAssets, GameAudioPlugin};
use ar_battle::{BattlePlugin, SpellsSheetSmall};
use ar_camera::ArenaCameraPlugin;
use ar_conf::{BG_COLOR, PFPS};
use ar_core::{
    AISet, AppState, AudioSet, BattleSet, CameraSet, InputSet, ItemsSet, MapSet, MonsterSet,
    ParticleSet, PlayerSet, SpellSet, UiSet, UtilSet,
};
use ar_enemies::MonsterSprites;
use ar_input::InputPlugin;
use ar_items::{ItemSheetSmall, ItemsPlugin};
use ar_map::{MapPlugin, TilesetHandle};
use ar_monsters::MonsterPlugin;
use ar_oneshot::OneShotPlugin;
use ar_particles::ParticlesPlugin;
use ar_player::{PlayerPlugin, SheetHandle};
use ar_spells::SpellsPlugin;
use ar_template::TemplatePlugin;
use ar_ui::{FontAssets, UiPlugin};
use ar_utils::UtilPlugin;

use bevy::{
    core::TaskPoolThreadAssignmentPolicy,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
    window::{PresentMode, WindowTheme},
};

use avian2d::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_fast_tilemap::plugin::FastTileMapPlugin;
use iyes_progress::ProgressPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rand::prelude::{EntropyPlugin, WyRand};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wpgu=off,bevy_render=off,bevy_ecs=trace,bevy_pbr=error,naga=warn,wgpu_hal=off"
                        .to_string(),
                    level: Level::INFO,
                    custom_layer: |_| None,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "arena2d".to_string(),
                        name: Some("arena2d".to_string()),
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
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
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                })
                .build(),
        )
        .init_state::<AppState>()
        .add_plugins(ProgressPlugin::new(AppState::LoadingAssets).continue_to(AppState::InBattle))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(FastTileMapPlugin::default())
        .add_plugins(OneShotPlugin)
        .add_plugins(ArenaCameraPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BattlePlugin)
        .add_plugins(MonsterPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(TemplatePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(UtilPlugin)
        .add_plugins(SpellsPlugin)
        .add_plugins(ParticlesPlugin)
        .add_plugins(ItemsPlugin)
        .add_plugins(PhysicsPlugins::new(FixedUpdate))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Time::<Fixed>::from_hz(PFPS))
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::LoadingTemplates)
                .load_collection::<MonsterSprites>()
                .load_collection::<SheetHandle>()
                .load_collection::<TilesetHandle>()
                .load_collection::<GameAudioAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<SpellsSheetSmall>()
                .load_collection::<ItemSheetSmall>()
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .insert_resource(Gravity(Vec2::ZERO))
        .configure_sets(
            Update,
            (
                CameraSet.run_if(in_state(AppState::InBattle)),
                PlayerSet.run_if(in_state(AppState::InBattle)),
                InputSet.run_if(in_state(AppState::InBattle)),
                MapSet.run_if(in_state(AppState::InBattle)),
                AudioSet.run_if(in_state(AppState::InBattle)),
                MonsterSet.run_if(in_state(AppState::InBattle)),
            ),
        )
        .configure_sets(
            FixedUpdate,
            (
                (AISet.run_if(in_state(AppState::InBattle))),
                (MonsterSet.run_if(in_state(AppState::InBattle))),
                (UiSet.run_if(in_state(AppState::InBattle))),
                (UtilSet.run_if(in_state(AppState::InBattle))),
                (BattleSet.run_if(in_state(AppState::InBattle))),
                (ParticleSet.run_if(in_state(AppState::InBattle))),
                (ItemsSet.run_if(in_state(AppState::InBattle))),
            ),
        )
        .configure_sets(OnEnter(AppState::InBattle), (UiSet).after(PlayerSet))
        .configure_sets(OnEnter(AppState::InBattle), (SpellSet).before(PlayerSet))
        .configure_sets(OnEnter(AppState::InBattle), (ParticleSet).after(UiSet));
    }
}
