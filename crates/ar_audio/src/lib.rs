use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::audio::PlaybackMode;
use bevy_asset_loader::prelude::*;
use ar_core::{AppState, AudioSet, ChangeBackgroundEvent};

#[derive(Debug, AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(paths(
        "audio/background/bg1.wav",
        "audio/background/bg2.wav",
        "audio/background/bg3.wav",
        "audio/background/bg4.wav"), collection(mapped, typed))]
    pub bg: HashMap<AssetFileStem, Handle<AudioSource>>,
    #[asset(paths(
        "audio/sfx/retro_lofi.wav",
        "audio/sfx/hit1.wav",
        "audio/sfx/death.wav",), collection(mapped, typed))]
    pub sfx: HashMap<AssetFileStem, Handle<AudioSource>>,
}

#[derive(Component)]
struct BGMusicMarker;

struct GameAudio {
}

pub struct GameAudioPlugin;

#[derive(Resource, Default)]
pub struct BackGroundMusic {
    max_bg: u8,
    current_bg: u8,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(AppState::InBattle),
                setup_bg.in_set(AudioSet),
            )
            .add_systems(Update,(
                (change_bgm).in_set(AudioSet),
                (play_music).in_set(AudioSet),
            ).chain());
    }
}

fn setup_bg(
    mut commands: Commands,
    audio_assets: Res<GameAudioAssets>,
    mut bgm: ResMut<BackGroundMusic>,) {
    commands.spawn(AudioBundle {
        source: audio_assets.bg.get("bg4").unwrap().clone().into(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        }
    })
    .insert(BGMusicMarker);
    bgm.max_bg = audio_assets.bg.len() as u8;
    bgm.current_bg = 4;
}

// Reads an event to change the background music,
// triggered either by a button press or by a timer
// despawning the current background music and changing the
// BackGroundMusic resource, which should trigger another system.
fn change_bgm(
    mut bgm: ResMut<BackGroundMusic>,
    bg_event: EventReader<ChangeBackgroundEvent>,
    current_bg: Query<(Entity, &AudioSink), With<BGMusicMarker>>,
    mut commands: Commands,
) {
    if !bg_event.is_empty() {
        for (entity, sink) in current_bg.iter() {
            sink.stop();
            commands.entity(entity).despawn();
        }
        if bgm.current_bg < bgm.max_bg {
            bgm.current_bg += 1;
        } else {
            bgm.current_bg = 1;
        }
    }
}

// System checks if BackGroundMusic resource is changed, 
// then spawns the new background music
fn play_music(
    bgm: Res<BackGroundMusic>,
    mut commands: Commands,
    audio_assets: Res<GameAudioAssets>,
) {
    if !bgm.is_changed() { return; }
    let bg = format!("bg{}", bgm.current_bg);
    commands.spawn(AudioBundle {
        source: audio_assets.bg.get(bg.as_str()).unwrap().clone().into(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        }
    })
    .insert(BGMusicMarker);
}