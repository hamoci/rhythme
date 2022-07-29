use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};
use crate::notes;


pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
        
        .add_startup_system(setup_audio_channel)
        .add_system(control_main_track)
        .add_audio_channel::<MainTrackChannel>()
        .add_audio_channel::<KeySoundChannel1>()
        .add_audio_channel::<KeySoundChannel2>()
        .add_audio_channel::<KeySoundChannel3>()
        .add_audio_channel::<KeySoundChannel4>();
    }
}

#[derive(Component, Default, Clone)]
pub struct MainTrackChannel;

#[derive(Component, Default, Clone)]
pub struct KeySoundChannel1;

#[derive(Component, Default, Clone)]
pub struct KeySoundChannel2;
#[derive(Component, Default, Clone)]
pub struct KeySoundChannel3;
#[derive(Component, Default, Clone)]
pub struct KeySoundChannel4;

pub struct ChannelAudioState<T> {
    stopped: bool,
    paused: bool,
    loop_started: bool,
    volume: f32,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for ChannelAudioState<T> {
    fn default() -> Self {
        ChannelAudioState {
            volume: 0.1,    //Basic : 1
            stopped: true,
            paused: false,
            loop_started: false,
            _marker: std::marker::PhantomData::<T>::default(),
        }
    }
}

pub struct AudioResource {
    main_track: Handle<AudioSource>,
    hit_sound1: Handle<AudioSource>,
    hit_sound2: Handle<AudioSource>,
    hit_sound3: Handle<AudioSource>,
    hit_sound4: Handle<AudioSource>
}

//init system
//Making Main SoundTrack Channel.(to play music)
pub fn setup_audio_channel(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let sound_track = asset_server.load("music/test.mp3");
    let hit1 = asset_server.load("music/hit_sound/key1.ogg");
    let hit2 = asset_server.load("music/hit_sound/key2.ogg");
    let hit3 = asset_server.load("music/hit_sound/key3.ogg");
    let hit4 = asset_server.load("music/hit_sound/key4.ogg");

    commands.insert_resource(AudioResource {
        main_track: sound_track,
        hit_sound1: hit1,
        hit_sound2: hit2,
        hit_sound3: hit3,
        hit_sound4: hit4
    });
    commands.insert_resource(ChannelAudioState::<MainTrackChannel>::default());
    commands.insert_resource(ChannelAudioState::<KeySoundChannel1>::default());
    commands.insert_resource(ChannelAudioState::<KeySoundChannel2>::default());
    commands.insert_resource(ChannelAudioState::<KeySoundChannel3>::default());
    commands.insert_resource(ChannelAudioState::<KeySoundChannel4>::default());
    
}

pub fn event_key_sound(

) {
    
}

pub fn control_main_track(
    audio_channel: Res<AudioChannel<MainTrackChannel>>,
    mut audio_state: ResMut<ChannelAudioState<MainTrackChannel>>,
    audio_source: Res<AudioResource>,
    hold_timer: Query<(&notes::MusicTimer, Without<notes::Hold>)>,
) {
    let (timer, _hold) = hold_timer.single();

    if timer.timer.elapsed_secs() > 0.{
        if audio_state.stopped == true {
            audio_channel.play(audio_source.main_track.clone());
            audio_channel.set_volume(audio_state.volume);
            audio_state.stopped = false;
            println!("Play Music");
            return
        }

        if timer.timer.paused() && !audio_state.paused{
            audio_channel.pause();
            audio_state.paused = true;
            println!("Music Paused");
        } else if !timer.timer.paused() && audio_state.paused{
            audio_channel.resume();
            audio_state.paused = false;
            println!("Music Resumed");
        }
    }
}