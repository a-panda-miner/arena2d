use ar_core::{AppState, AudioSet, BGMusicMarker, ChangeBackgroundEvent, Cooldown};
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

#[derive(Debug, AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(
        paths(
            "audio/background/bg1.wav",
            "audio/background/bg2.wav",
            "audio/background/bg3.wav",
            "audio/background/bg4.wav"
        ),
        collection(mapped, typed)
    )]
    pub bg: HashMap<AssetFileStem, Handle<AudioSource>>,
    #[asset(
        paths(
            "audio/sfx/retro_lofi.wav",
            "audio/sfx/hit1.wav",
            "audio/sfx/death.wav",
        ),
        collection(mapped, typed)
    )]
    pub sfx: HashMap<AssetFileStem, Handle<AudioSource>>,
}

#[derive(Resource)]
pub struct BackGroundMusic {
    max_bg: usize,
    current_bg: usize,
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InBattle),
            (setup_flat_bg, setup_bg).chain().in_set(AudioSet),
        )
        .add_systems(
            Update,
            ((change_bgm).in_set(AudioSet), (play_music).in_set(AudioSet)).chain(),
        );
    }
}

#[derive(Resource)]
struct FlatBGList {
    list: Vec<Handle<AudioSource>>,
}

fn setup_flat_bg(mut commands: Commands, audio_assets: Res<GameAudioAssets>) {
    let mut list = Vec::new();
    for handle in audio_assets.bg.values() {
        list.push(handle.clone());
    }
    commands.insert_resource(FlatBGList { list });
}

fn setup_bg(mut commands: Commands, audio_assets: Res<FlatBGList>) {
    let bgm = BackGroundMusic {
        max_bg: audio_assets.list.len(),
        current_bg: 0,
    };
    commands
        .spawn(AudioBundle {
            source: audio_assets.list[bgm.current_bg].clone().into(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        })
        .insert(BGMusicMarker)
        .insert(Cooldown(Timer::from_seconds(40., TimerMode::Repeating)));
    commands.insert_resource(bgm);
}

/*
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
    .insert(BGMusicMarker)
    .insert(Cooldown(Timer::from_seconds(40., TimerMode::Repeating)));
    bgm.max_bg = audio_assets.bg.len() as u8;
    bgm.current_bg = 4;
}
*/
// Reads an event to change the background music,
// triggered either by a button press or by a timer
// despawning the current background music and changing the
// BackGroundMusic resource, which should trigger another system.
fn change_bgm(
    mut bgm: ResMut<BackGroundMusic>,
    bg_event: EventReader<ChangeBackgroundEvent>,
    current_bgm: Query<(Entity, &AudioSink), With<BGMusicMarker>>,
    mut commands: Commands,
) {
    if !bg_event.is_empty() {
        for (entity, sink) in current_bgm.iter() {
            sink.stop();
            commands.entity(entity).despawn();
        }
        bgm.current_bg = (bgm.current_bg + 1) % bgm.max_bg;
    }
}

// System checks if BackGroundMusic resource is changed,
// then spawns the new background music
fn play_music(bgm: Res<BackGroundMusic>, mut commands: Commands, audio_assets: Res<FlatBGList>) {
    if !bgm.is_changed() {
        return;
    }
    let bg = bgm.current_bg;
    commands
        .spawn(AudioBundle {
            source: audio_assets.list[bg].clone().into(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        })
        .insert(BGMusicMarker)
        .insert(Cooldown(Timer::from_seconds(40., TimerMode::Repeating)));
}
