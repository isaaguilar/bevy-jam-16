use bevy::{audio::Volume, prelude::*};

use crate::{assets::GameAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );

    app.add_systems(OnEnter(Screen::Gameplay), start_music);
    app.add_systems(OnExit(Screen::Gameplay), pause_music);
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.5)),
        Music,
    )
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

pub fn start_music(
    mut commands: Commands,
    music_query: Query<&AudioSink, With<Music>>,
    game_assets: Res<GameAssets>,
) {
    if let Ok(audio) = music_query.single() {
        audio.play();
    } else {
        commands.spawn((
            Name::new("Gameplay Music"),
            music(game_assets.tubamusic.clone()),
        ));
    }
}

pub fn pause_music(music_query: Query<&AudioSink, With<Music>>) {
    if let Ok(audio) = music_query.single() {
        audio.pause();
    }
}
