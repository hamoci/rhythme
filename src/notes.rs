use bevy::prelude::*;

struct NoteResource {
    judge: Handle<ColorMaterial>,
    note_first: Handle<ColorMaterial>,
    note_second: Handle<ColorMaterial>,
    note_third: Handle<ColorMaterial>,
    note_fourth: Handle<ColorMaterial>,
}
impl FromWorld for NoteResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let first_handle = asset_server.load("note_first.png");
        let second_handle = asset_server.load("note_second.png");
        let third_handle = asset_server.load("note_third.png");
        let fourth_handle = asset_server.load("note_fourth.png");
        let judge_handle = asset_server.load("judge.png");
    

        NoteResource {
             judge: material.add(judge_handle.into()),
             note_first: material.add(first_handle.into()),
             note_second: material.add(second_handle.into()),
             note_third: material.add(third_handle.into()),
             note_fourth: material.add(fourth_handle.into()),
        }
    }
}

enum Press4Key{
    First,
    Second,
    Third,
    Fourth,
}

enum NoteType{
    Long,
    Short,
}

#[derive(Component)]
struct Note {
    note_type: NoteType,
    press_key: Press4Key,
}

//Timer를 하나 만들고, audio읽어서 몇분짜리인지 확인. 그 후 File에서 채보를 불러옴
//File에 audio, 채보, audio info에 대해 넣어야할듯
//채보는 text파일로 하여 stack이나 vector에 다 때려박으면 될듯
fn open_chart() {
}