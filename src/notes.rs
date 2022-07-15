use bevy::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct NoteResource {
    judge: Handle<Image>,
    background: Handle<Image>,
    note_first: Handle<Image>,
    note_second: Handle<Image>,
    note_third: Handle<Image>,
    note_fourth: Handle<Image>,
}

pub enum Press4Key{
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

pub enum NoteType{
    Long = 0,
    Short = 1,
}

#[derive(Component)]
pub struct Note {
    note_type: NoteType,
    press_key: Press4Key,
    timing: usize,
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
        
    }
}

pub fn playing_setup(
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

pub fn spawn_note(
    commands: &mut Commands,
    materials: Res<NoteResource>,
    time: Res<Time>,
    note: Res<Note>,
    mut timer: ResMut<MusicTimer>,
) {
    let material = match note.press_key {
        Press4Key::First => materials.note_first.clone(),
        Press4Key::Second => materials.note_second.clone(),
        Press4Key::Third => materials.note_third.clone(),
        Press4Key::Fourth => materials.note_fourth.clone(),
    };

    commands.spawn_bundle(SpriteBundle {
        texture: material,
        ..Default::default()
    });
}

pub struct Chart(Vec<Note>);

pub struct MusicTimer(Timer);

pub fn playing_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let music = asset_server.load("music/test.ogg");
    audio.play(music);
}

// #채보 Vec<Note>에 다 박아놓고 반환
pub fn open_chart(commands:&mut Commands){
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
    commands.insert_resource(Chart(chart_vec));
}

fn parse_file_string(string: &String) -> Result<Note, &'static str> {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut commas: usize = 0;
    let mut note = Note {
        note_type: NoteType::Short,
        press_key: Press4Key::First,
        timing: 0,
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