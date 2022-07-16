use bevy::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const FRAME: f32 = 1.0/60.0;

pub struct NoteResource {
    judge: Handle<Image>,
    background: Handle<Image>,
    note_first: Handle<Image>,
    note_second: Handle<Image>,
    note_third: Handle<Image>,
    note_fourth: Handle<Image>,
    line: Handle<Image>,
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
            background: asset_server.load("image/background"),
            line: asset_server.load("image/line.png"),
        };

        note_resource
    }
}

#[derive(Clone)]
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

pub struct SongInfo {
    name: String,
    time_length: f32,
    difficulty: f32,
}

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
//Timer를 하나 만들고, audio읽어서 몇분짜리인지 확인. 그 후 File에서 채보를 불러옴
//File에 audio, 채보, audio info에 대해 넣어야할듯

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoteResource>()
            .add_startup_system(playing_setup)
            .add_startup_system(open_chart)
            .add_system(spawn_note)
            .add_system(move_note)
            .add_system(show_playing_timer);
    }
}

/*
Useless Code
pub fn load_note_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let note_resource = NoteResource {
        judge: asset_server.load("image/judge.png"),
        note_first: asset_server.load("image/note_first.png"),
        note_second: asset_server.load("image/note_second.png"),
        note_third: asset_server.load("image/note_third.png"),
        note_fourth: asset_server.load("image/note_fourth.png"),
        background: asset_server.load("image/background"),
    };

    commands.insert_resource(note_resource);
}
*/
pub fn playing_setup(
    mut commands: Commands,
    materials: Res<NoteResource>
) {
    
    commands.spawn_bundle(SpriteBundle {
        texture: materials.judge.clone(),
        transform: Transform::from_translation(Vec3::new(0., -350., 0.)),
        ..Default::default()
    });

    for i in -2..3 {
        commands.spawn_bundle(SpriteBundle {
            texture: materials.line.clone(),
            transform: Transform::from_translation(Vec3::new(i as f32 * 101., 0., 0.)),
            ..Default::default()
        });
    }
    let mut timer = MusicTimer(Timer::from_seconds(100., false));
    commands.insert_resource(timer);
}

pub fn show_playing_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MusicTimer>
) {
    timer.0.tick(std::time::Duration::from_secs_f32(time.delta_seconds()));
    println!("{}", timer.0.elapsed_secs());  
}

pub fn spawn_note(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &Chart)>
) {

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
            
            let position_y: f32 = -350. + ((note.timing as f32) / 1000.) * (100. * note.speed);

            let position = Transform::from_translation(Vec3::new(position_x, position_y, 1.));

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
    time: Res<Time>
) {
    for (_entity, note, mut transform) in query_note.iter_mut() {
        transform.translation.y -= time.delta_seconds() * 100. * note.speed;
    }
}

#[derive(Component)]
pub struct Chart {
    notes: Vec<Note>
}

#[derive(Component)]
pub struct Speed(f32);

pub struct MusicTimer(Timer);

pub fn playing_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let music = asset_server.load("music/test.ogg");
    audio.play(music);
}

// #채보 Vec<Note>에 다 박아놓고 반환
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
    let mut chart_component = commands.spawn();
    chart_component.insert(chart);
}

fn parse_file_string(string: &String) -> Result<Note, &'static str> {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut commas: usize = 0;
    let mut note = Note {
        note_type: NoteType::Short,
        press_key: Press4Key::First,
        timing: 0,
        speed: 7.0,
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

//노트가 화면 맨 위에서 화면 맨아래까지 얼마나 걸릴지를 속도로 정하면 될듯