use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const FRAME: f32 = 1.0/60.0;
const STANDARD_NOTE_SPEED: f32 = 100.;

pub struct FontResource {
    font: Handle<Font>,
}

impl FromWorld for FontResource {
    fn from_world(world: &mut World) -> Self {

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let font_resource = FontResource {
            font: asset_server.load("font/Galmuri11.ttf")
        };

        font_resource
    }
}

pub struct NoteResource {
    judge: Handle<Image>,
    background: Handle<Image>,
    note_first: Handle<Image>,
    note_second: Handle<Image>,
    note_third: Handle<Image>,
    note_fourth: Handle<Image>,
    backlight: Handle<Image>,
    line: Handle<Image>,
    pause: Handle<Image>,
}

impl FromWorld for NoteResource {
    fn from_world(world: &mut World) -> Self {

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let note_resource = NoteResource {
            judge: asset_server.load("image/judge.png"),
            note_first: asset_server.load("image/note_first.png"),
            note_second: asset_server.load("image/note_second.png"),
            note_third: asset_server.load("image/note_third.png"),
            note_fourth: asset_server.load("image/note_fourth.png"),
            background: asset_server.load("image/background.png"),
            backlight: asset_server.load("image/backlight.png"),
            line: asset_server.load("image/line.png"),
            pause: asset_server.load("image/pause.png"),
        };

        note_resource
    }
}

#[derive(Eq, PartialEq, Component, Clone)]
pub enum Press4Key{
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

#[derive(Clone)]
pub enum NoteType{
    Long = 0,
    Short = 1,
}

#[derive(Component, Clone)]
pub struct Note {
    note_type: NoteType,
    press_key: Press4Key,
    timing: usize,
    speed: f32,
}

#[derive(Component)]
pub struct BackLight;
pub struct SongInfo {
    name: String,
    bpm: usize,
    time_length: f32,
    difficulty: f32,
}

#[derive(Component)]
pub struct PlayingInfo {
    song_name: String,
    accuracy: f32,
    score: usize,
    current_time: f32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, StageLabel)]
pub enum GameStage {
    Playing,
    Select,
}

#[derive(Component)]
pub struct Chart {
    notes: Vec<Note>
}

#[derive(Component)]
pub struct MusicTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Hold;

#[derive(Component)]
pub struct PausedText;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct Scoreboard {
    perfect: usize,
    great: usize,
    miss: usize,
}

//Timer를 하나 만들고, audio읽어서 몇분짜리인지 확인. 그 후 File에서 채보를 불러옴
//File에 audio, 채보, audio info에 대해 넣어야할듯

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoteResource>()
            .init_resource::<FontResource>()
            .add_audio_channel::<MainTrackChannel>()
            .add_startup_system(setup_audio_channel)
            .add_startup_system(setup_background_text)
            .add_startup_system(spawn_background)
            .add_startup_system(open_chart)
            .add_system(game_ticking)
            .add_system(update_background_text)
            .add_system(update_scoreboard)

            .add_system(control_audio)

            .add_system(spawn_note)
            .add_system(despawn_note)
            .add_system(move_note)

            .add_system(spawn_keyboard_backlight)
            .add_system(despawn_keyboard_backlight)

            .add_system(pause_game);
           // app.add_system(_show_playing_timer); // for debug
    }
}

pub fn spawn_background(
    mut commands: Commands,
    materials: Res<NoteResource>
) {
    commands.spawn_bundle(SpriteBundle {
        texture: materials.background.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: materials.judge.clone(),
        transform: Transform::from_translation(Vec3::new(0., -350., 2.)),
        ..Default::default()
    });

    for i in -2..3 {
        commands.spawn_bundle(SpriteBundle {
            texture: materials.line.clone(),
            transform: Transform::from_translation(Vec3::new(i as f32 * 101., 0., 1.)),
            ..Default::default()
        });
    }
}

pub fn game_ticking(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Query<(Entity, &mut MusicTimer, Without<Hold>)>,
    mut hold_time: Query<(Entity, &mut MusicTimer, With<Hold>)>
) {
    for (entity, mut music_time, _hold) in hold_time.iter_mut() {
        music_time.timer.tick(time.delta());
        //println!("Ticking(hold): {}", music_time.timer.elapsed_secs()); //for debug
        if music_time.timer.finished() {
            commands.entity(entity).despawn();
        }
        return
    }

    //Timer ticking
    //현재 시간을 나타내는 Time을 불러온 뒤, 현재와 마지막 tick간의 시간차만큼 timer가 흐르게 만듬
    //주석처리된 코드는 Duration structure를 반환하기 때문에, 필요에 따라 아래의 코드를 써야할 때도 있음
    //music_timer.timer.tick(std::time::Duration::from_secs_f32(time.delta_seconds()));
    for (_entity, mut music_timer, _dummy) in timer.iter_mut() {    
        if !music_timer.timer.paused() {
            music_timer.timer.tick(time.delta());
        }
    }
}

pub fn spawn_note(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &Chart)>
) {
    //println!("")
    for chart in query_entity.iter_mut() {
        for note in chart.1.notes.iter() {
            let material = match note.press_key {
                Press4Key::First => materials.note_first.clone(),
                Press4Key::Second => materials.note_second.clone(),
                Press4Key::Third => materials.note_third.clone(),
                Press4Key::Fourth => materials.note_fourth.clone(),
            };

            let position_x: f32 = match note.press_key {
                Press4Key::First => -151.5,
                Press4Key::Second => -50.5,
                Press4Key::Third => 50.5,
                Press4Key::Fourth => 151.5,
            };
            // -350 : Judgement line
            let position_y: f32 = -350. + ((note.timing as f32) / 1000.) * (STANDARD_NOTE_SPEED * note.speed);

            let position = Transform::from_translation(Vec3::new(position_x, position_y, 3.));

            commands.spawn_bundle(SpriteBundle {
                texture: material,
                transform: position,
                ..Default::default()
            }).insert(Note {
                note_type: note.note_type.clone(),
                press_key: note.press_key.clone(),
                timing: note.timing,
                speed: note.speed,
            });
        }
    }
    for chart in query_entity.iter_mut() {
        commands.entity(chart.0).despawn();
    }

}

pub fn move_note(
    mut query_note: Query<(Entity, &Note, &mut Transform)>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    time: Res<Time>
) {
    for (_entity, music_timer, _dummy) in timer.iter() {
        for (_entity, note, mut transform) in query_note.iter_mut() {
            if (music_timer.timer.elapsed_secs() > 0.) && (!music_timer.timer.paused()) {
                transform.translation.y -= time.delta_seconds() * STANDARD_NOTE_SPEED * note.speed;
            }
        }
    }
}



pub fn despawn_note(
    mut commands: Commands,
    query_note: Query<(Entity, &Note)>,
    key_input: Res<Input<KeyCode>>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    mut score: Query<&mut Scoreboard>
) {
    for (_entity, music_timer, _dummy) in timer.iter() {
        for (entity, note) in query_note.iter() {
            let key: KeyCode = match note.press_key {
                Press4Key::First => KeyCode::Z,
                Press4Key::Second => KeyCode::X,
                Press4Key::Third => KeyCode::Period,
                Press4Key::Fourth => KeyCode::Slash,
                _ => KeyCode::Key0
            };

            let mut score = score.single_mut();

            //Judgement : Perfect 0.04167sec (DJMAX V Respect)
            //            Great   0.09000sec
            if key_input.just_pressed(key) && (!music_timer.timer.paused()) {
                //println!("current timer: {}", music_timer.timer.elapsed_secs());
                if (note.timing as f32 / 1000. + 0.04167 > music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.04167 < music_timer.timer.elapsed_secs()) {
                    println!("note timing : {}", note.timing as f32 / 1000.);
                    commands.entity(entity).despawn();
                    score.perfect += 1;
                    println!("perfect {}", note.timing as f32 / 1000.);
                } else if (note.timing as f32 / 1000. + 0.09 > music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.09 < music_timer.timer.elapsed_secs()) {
                    println!("note timing : {}", note.timing as f32 / 1000.);
                    commands.entity(entity).despawn();
                    score.great += 1;
                    println!("great {}", note.timing as f32 / 1000.);
                }
            }
            
            if note.timing as f32 / 1000. + 0.09  < music_timer.timer.elapsed_secs() {
                commands.entity(entity).despawn();
                score.miss += 1;
                println!("miss {}", music_timer.timer.elapsed_secs());
            }
        }
    } 
}

pub fn spawn_keyboard_backlight(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    materials: Res<NoteResource>, 
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>
) {
    let mut is_paused: bool = false;
    for (_entity, music_timer, _dummy) in timer.iter() {
        is_paused = music_timer.timer.paused();
    }
    if !is_paused {
        if key_input.just_pressed(KeyCode::Z) {
            commands.spawn_bundle(SpriteBundle {
                texture: materials.backlight.clone(),
                transform: Transform::from_translation(Vec3::new(-151.5, 75., 1.)),
                ..Default::default()
            }).insert(Press4Key::First).insert(BackLight);
        }
        if key_input.just_pressed(KeyCode::X) {
            commands.spawn_bundle(SpriteBundle {
                texture: materials.backlight.clone(),
                transform: Transform::from_translation(Vec3::new(-50.5, 75., 1.)),
                ..Default::default()
            }).insert(Press4Key::Second).insert(BackLight);
        }
        if key_input.just_pressed(KeyCode::Period) {
            commands.spawn_bundle(SpriteBundle {
                texture: materials.backlight.clone(),
                transform: Transform::from_translation(Vec3::new(50.5, 75., 1.)),
                ..Default::default()
            }).insert(Press4Key::Third).insert(BackLight);
        }
        if key_input.just_pressed(KeyCode::Slash) {
            commands.spawn_bundle(SpriteBundle {
                texture: materials.backlight.clone(),
                transform: Transform::from_translation(Vec3::new(151.5, 75., 1.)),
                ..Default::default()
            }).insert(Press4Key::Fourth).insert(BackLight);
        }
    }
}

pub fn despawn_keyboard_backlight(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    query : Query<(Entity, &BackLight, &Press4Key)>,
) {
    for (entity, _backlight, key_type) in query.iter() {
        if key_input.just_released(KeyCode::Z) && (*key_type == Press4Key::First){
            commands.entity(entity).despawn();
        }
        if key_input.just_released(KeyCode::X) && (*key_type == Press4Key::Second){
            commands.entity(entity).despawn();
        }
        if key_input.just_released(KeyCode::Period) && (*key_type == Press4Key::Third){
            commands.entity(entity).despawn();
        }
        if key_input.just_released(KeyCode::Slash) && (*key_type == Press4Key::Fourth){
            commands.entity(entity).despawn();
        }
    }
}

pub fn _print_keyboard_event_system(mut keyboard_input_events: EventReader<bevy::input::keyboard::KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

#[derive(Component, Default, Clone)]
pub struct MainTrackChannel;

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
}

pub fn setup_background_text(
    mut commands: Commands,
    font_resource: Res<FontResource>,
) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        text: Text { 
            sections: vec![
                TextSection {
                    value: "Time : ".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                        ..default()
                    },
                },
                
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                }
            ],
            ..default()
        },
        //transform: Transform::from_translation(Vec3::new(-350., 450., 10.)),
        ..default()
    })
    .insert(TimerText);

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::FlexEnd,
            position: bevy::math::Rect::<bevy::ui::Val> { top: Val::Px(30.), ..Default::default() },
            ..default()
        },
        text: Text { 
            sections: vec![
                TextSection {
                    value: "Perfect : ".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                        ..default()
                    },
                },
                
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                },

                TextSection {
                    value: "\nGreat : ".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                },

                TextSection {
                    value: "\nMiss : ".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 20.0,
                        color: Color::GOLD,
                    }
                }
            ],
            ..default()
        },
        //transform: Transform::from_translation(Vec3::new(-350., 450., 10.)),
        ..default()
    }).insert(Scoreboard {perfect:0, great:0, miss:0});

}

pub fn update_background_text(
    mut query: Query<(&mut Text, With<TimerText>)>,
    timer: Query<(&MusicTimer, Without<Hold>)>,
) {
    let (music_timer, _hold) = timer.single();
    for (mut time_text, _timer_text) in query.iter_mut() {
        time_text.sections[1].value = music_timer.timer.elapsed_secs().to_string();
    }
}

pub fn update_scoreboard(
    mut score_query: Query<(&mut Text, &Scoreboard)>
) {
        let (mut score_text, scoreboard) = score_query.single_mut();
        score_text.sections[1].value = scoreboard.perfect.to_string();
        score_text.sections[3].value = scoreboard.great.to_string();
        score_text.sections[5].value = scoreboard.miss.to_string();
}

//init system
//Making Main SoundTrack Channel.(to play music)
pub fn setup_audio_channel(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let sound_track = asset_server.load("music/test.mp3");

    commands.insert_resource(AudioResource {main_track: sound_track});
    commands.insert_resource(ChannelAudioState::<MainTrackChannel>::default());
    
}

pub fn control_audio(
    mut commands: Commands,
    audio_channel: Res<AudioChannel<MainTrackChannel>>,
    mut audio_state: ResMut<ChannelAudioState<MainTrackChannel>>,
    audio_source: Res<AudioResource>,
    hold_timer: Query<(&MusicTimer, Without<Hold>)>,
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

//for debug
pub fn _show_playing_timer(
    mut timer: Query<(Entity, &MusicTimer, Without<Hold>)>
) {
    for (_entity, music_timer, _dummy) in timer.iter() {
        println!("{}", music_timer.timer.elapsed_secs());  
    }
}

pub fn pause_game(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mut timer: Query<(Entity, &mut MusicTimer)>,
    mut text: Query<(Entity, &PausedText)>,
    materials: Res<NoteResource>
) {
    for (_entity, mut music_timer) in timer.iter_mut() {
        if key_input.just_pressed(KeyCode::Escape) && (!music_timer.timer.paused()) {
            music_timer.timer.pause();
            //TODO: 차후에 Font 추가후 텍스트로 바꿔야함
            commands.spawn_bundle( SpriteBundle {
                texture: materials.pause.clone(),
                transform: Transform::from_translation(Vec3::new(275., 0., 3.)),
                ..Default::default()
            }).insert(PausedText);
        } else if key_input.just_pressed(KeyCode::Escape) && music_timer.timer.paused() {
            music_timer.timer.unpause();
            for (entity, paused_text) in text.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn open_chart(mut commands: Commands) {
    let chart_file = File::open("assets/music/test.txt").expect("file not found");
    let mut chart_vec: Vec<Note> = Vec::new();
    let mut buffer = BufReader::new(chart_file);
    let mut line: String = String::new();

    loop {
        let read_bytes = buffer.read_line(&mut line).unwrap();
        println!("Buffer: {}", line.trim());
        println!("Read Bytes: {}", read_bytes);
        if read_bytes == 0 {
            break;
        }
        chart_vec.push(parse_file_string(&line).unwrap());
        line.clear();
    }
    chart_vec.sort_by(|a, b| a.timing.cmp(&b.timing));
    let chart = Chart {
        notes: chart_vec,
    };
    commands.spawn().insert(chart);

    let music_timer = MusicTimer {timer: Timer::from_seconds(200., false)};
    commands.spawn().insert(music_timer);

    //게임시작하고 3초대기
    let hold_timer = MusicTimer { timer: Timer::from_seconds(3., false)}; 
    commands.spawn().insert(hold_timer).insert(Hold);

}

fn parse_file_string(string: &String) -> Result<Note, &'static str> {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut commas: usize = 0;
    let mut note = Note {
        note_type: NoteType::Short,
        press_key: Press4Key::First,
        timing: 0,
        speed: 17.0,
    };
    println!("parsed string: {}", string.trim());
    for c in string.chars() {
        if c == ',' {
            match commas {
                0 => { 
                    let parsed: usize = (&string[start..end]).parse().unwrap();
                    start = end + 1;
                    println!("commas 0: {}", parsed);
                    match parsed {
                        0 => note.press_key = Press4Key::First,
                        1 => note.press_key = Press4Key::Second,
                        2 => note.press_key = Press4Key::Third,
                        3 => note.press_key = Press4Key::Fourth,
                        _ => panic!("parsing press_key error")
                    }
                }
                
                1 => {
                    let parsed: &str = &string[start..end];
                    start = end + 1;
                    println!("commas 1: {}", parsed);
                    match parsed {
                        "Short" => note.note_type = NoteType::Short,
                        "Long" => note.note_type = NoteType::Long,
                        _ => panic!("parsing type error")
                    }
                }

                2 => {
                    let parsed: usize = (&string[start..end]).parse().unwrap();
                    println!("commas 2: {}\n", parsed);
                    note.timing = parsed;
                }
                3 => break,
                _ => panic!("parsing comma error : {}", commas),
            }
            commas = commas + 1;
        } 
        end = end + 1;
    }

    if commas == 3 {
        Ok(note)
    } else {
        Err("Not parsed")
    }
} 