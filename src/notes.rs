use bevy::prelude::*;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use core::time;
use crate::state::GameState;

const STANDARD_NOTE_SPEED: f32 = 100.;
const HOLD_TIME: f32 = 3000.;
const MAX_MUSIC_LENGTH: f32 = 600000.;
const JUDGE_LINE: f32 = -250.;
const FIRST_KEY: KeyCode = KeyCode::Z;
const SECOND_KEY: KeyCode = KeyCode::X;
const THIRD_KEY: KeyCode = KeyCode::Period;
const FOURTH_KEY: KeyCode = KeyCode::Slash;

pub struct FontResource {
    pub font: Handle<Font>,
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

pub struct JudgeResource {
    perfect: Handle<Image>,
    great: Handle<Image>,
    miss: Handle<Image>,
    bad: Handle<Image>,
}

impl FromWorld for JudgeResource {
    fn from_world(world: &mut World) -> Self {

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let judge_resource = JudgeResource {
            perfect: asset_server.load("image/perfect.png"),
            great: asset_server.load("image/great.png"),
            miss: asset_server.load("image/miss.png"),
            bad: asset_server.load("image/bad.png"),
        };

        judge_resource        
    }
}

/*
pub struct NumberResource {
    zero: Handle<Image>,
    one: Handle<Image>,
    two: Handle<Image>,
    three: Handle<Image>,
    four: Handle<Image>,
    five: Handle<Image>,
    six: Handle<Image>,
    seven: Handle<Image>,
    eight: Handle<Image>,
    nine: Handle<Image>,
}

impl FromWorld for NumberResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let number_resource = NumberResource {
            zero: asset_server.load("image/number/0.png"),
            one: asset_server.load("image/number/1.png"),
            two: asset_server.load("image/number/2.png"),
            three: asset_server.load("image/number/3.png"),
            four: asset_server.load("image/number/4.png"),
            five: asset_server.load("image/number/5.png"),
            six: asset_server.load("image/number/6.png"),
            seven: asset_server.load("image/number/7.png"),
            eight: asset_server.load("image/number/8.png"),
            nine: asset_server.load("image/number/9.png"),
        };

        number_resource
    }
}

*/
#[derive(Eq, PartialEq, Component, Clone)]
pub enum Press4Key{
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

#[derive(Clone, Eq, PartialEq)]
pub enum NoteType{
    Long = 0,
    Short = 1,
}

#[derive(Component, Clone)]
pub struct Note {
    note_type: NoteType,
    press_key: Press4Key,
    timing: usize,
    release_timing: usize,
    pushed: bool,
    missed: bool,
    speed: f32,
    judge: JudgeAccuracy,
}

#[derive(Component)]
pub struct BackLight;

#[derive(Component)]
pub struct Chart {
    notes: VecDeque<Note>
}

#[derive(Component)]
pub struct MusicTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Hold;

#[derive(Component)]
pub struct PausedText;

//수정필요
#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct Scoreboard {
    perfect: usize,
    great: usize,
    miss: usize,
    bad: usize,
}

#[derive(Component)]
pub struct LongNoteTimer {
    timer: Timer,
}
#[derive(Component)]
pub enum Lane {
    First,
    Second,
    Third,
    Fourth,
    None,
}

#[derive(Component)]
pub struct FirstLane;
#[derive(Component)]
pub struct SecondLane;
#[derive(Component)]
pub struct ThirdLane;
#[derive(Component)]
pub struct FourthLane;

#[derive(Component)]
pub struct Combo(u32);

//차후에 combo animation 구현 후 삭제
#[derive(Component)]
pub struct ComboText;

#[derive(Component)]
pub struct TotalAccuracy(f32);

#[derive(Component)]
pub struct AccuracyText;

pub struct EventCombo {
    judge: JudgeAccuracy,
}

pub struct EventAnimation {
    judge: JudgeAccuracy,
}

pub struct KeySound1;
pub struct KeySound2;
pub struct KeySound3;
pub struct KeySound4;


pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NoteResource>()
            .init_resource::<FontResource>()
            .init_resource::<JudgeResource>()
            //.init_resource::<NumberResource>()

            .add_event::<EventAnimation>()
            .add_event::<KeySound1>()
            .add_event::<KeySound2>()
            .add_event::<KeySound3>()
            .add_event::<KeySound4>()
            .add_event::<EventCombo>()

            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                .with_system(setup_background_text)
                .with_system(spawn_background)
                .with_system(open_chart)
                .with_system(setup_accuracy)
                .with_system(setup_combo)
                .with_system(spawn_long_note_timer)
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                .with_system(game_ticking)
                .with_system(update_background_text)
                .with_system(update_scoreboard)
                .with_system(update_accuracy)
    
                .with_system(spawn_note_0)
                .with_system(spawn_note_1)
                .with_system(spawn_note_2)
                .with_system(spawn_note_3)
    
                .with_system(despawn_note_0)
                .with_system(despawn_note_1)
                .with_system(despawn_note_2)
                .with_system(despawn_note_3)
    
                .with_system(move_note)
    
                .with_system(spawn_keyboard_backlight)
                .with_system(despawn_keyboard_backlight)
    
                .with_system(spawn_judgement)
                .with_system(update_judgement)
    
                .with_system(update_combo_effect)
    
                .with_system(pause_game)
                /* Debug Only */
                .with_system(print_keyboard_event_system)
            );

           
    }
}

pub fn spawn_long_note_timer(
    mut commands: Commands,
) {
    commands.spawn().insert(LongNoteTimer{timer: Timer::from_seconds(0.25, true)}).insert(FirstLane);
    commands.spawn().insert(LongNoteTimer{timer: Timer::from_seconds(0.25, true)}).insert(SecondLane);
    commands.spawn().insert(LongNoteTimer{timer: Timer::from_seconds(0.25, true)}).insert(ThirdLane);
    commands.spawn().insert(LongNoteTimer{timer: Timer::from_seconds(0.25, true)}).insert(FourthLane);
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
        transform: Transform::from_translation(Vec3::new(0., JUDGE_LINE, 2.)),
        ..Default::default()
    });

    for i in -2..3 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite { color: Color::rgba(0.98, 0.98, 0.98, 0.05), ..Default::default()},
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

//-151.5
pub fn spawn_note_0(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &mut Chart, With<FirstLane>)>,
    timer: Query<(&MusicTimer, Without<Hold>)>,
) {

    for (entity, mut query, _lane) in query_entity.iter_mut() {
        let (music_timer, _hold) = timer.single();
        let material = materials.note_first.clone();
        if query.notes.is_empty() {
            commands.entity(entity).despawn();
            return;
        }
        spawn_note(&mut commands, material, &mut query, -151.5, music_timer);
    }

}

//-50.5
pub fn spawn_note_1(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &mut Chart, With<SecondLane>)>,
    timer: Query<(&MusicTimer, Without<Hold>)>, 
) {
    for (entity, mut query, _lane) in query_entity.iter_mut() {
        let (music_timer, _hold) = timer.single();
        let material = materials.note_second.clone();
        if query.notes.is_empty() {
            commands.entity(entity).despawn();
            return;
        }
        spawn_note(&mut commands, material, &mut query, -50.5, music_timer);
    }
}

//50.5
pub fn spawn_note_2(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &mut Chart, With<ThirdLane>)>,
    timer: Query<(&MusicTimer, Without<Hold>)>,
) {
    for (entity, mut query, _lane) in query_entity.iter_mut() {
        let (music_timer, _hold) = timer.single();
        let material = materials.note_third.clone();
        if query.notes.is_empty() {
            commands.entity(entity).despawn();
            return;
        }
        spawn_note(&mut commands, material, &mut query, 50.5, music_timer);
    }
}

//151.5
pub fn spawn_note_3(
    mut commands: Commands,
    materials: Res<NoteResource>,
    mut query_entity: Query<(Entity, &mut Chart, With<FourthLane>)>,
    timer: Query<(&MusicTimer, Without<Hold>)>,
) {
    for (entity, mut query, _lane) in query_entity.iter_mut() {
        let (music_timer, _hold) = timer.single();
        let material = materials.note_fourth.clone();
        if query.notes.is_empty() {
            commands.entity(entity).despawn();
            return;
        }
        spawn_note(&mut commands, material, &mut query, 151.5, music_timer);
    }
}

fn spawn_note(
    commands: &mut Commands,
    material: Handle<Image>,
    chart: &mut Chart,
    position_x: f32,
    timer: &MusicTimer,
) {
    // -350 : Judgement line. (STANDARD_NOTE_SPEED * note.speed) = 1초에 움직이는 거리. 즉 생성할때 chart.notes[0].timing / 1000초만큼 이동해야 판정선에 닿도록 함
    // y가 530보다 큰 것은 생성하지 않음
    let position_y: f32 = JUDGE_LINE + (((chart.notes[0].timing as f32) / 1000.) - timer.timer.elapsed_secs())
     * (STANDARD_NOTE_SPEED * chart.notes[0].speed);

    if position_y <= 530.{
        //println!("Note spawned");
        //println!("{}", position_y);

        if chart.notes[0].note_type == NoteType::Long {
            let release_position: f32 = position_y + ((chart.notes[0].release_timing - chart.notes[0].timing) as f32 / 1000.)
             * (STANDARD_NOTE_SPEED * chart.notes[0].speed);
            let mut position = Transform::from_translation(Vec3::new(position_x, position_y + (release_position - position_y) / 2., 3.));
            let scale = 1. + (1. / 30. * (release_position - position_y));
            let scale = Scale(scale);
            position.scale = Vec3::new(1., scale.0, 1.);
            commands.spawn_bundle(SpriteBundle {
                texture: material,
                transform: position,
                ..Default::default()
            }).insert(Note {
                note_type: chart.notes[0].note_type.clone(),
                press_key: chart.notes[0].press_key.clone(),
                release_timing: chart.notes[0].release_timing,
                timing: chart.notes[0].timing,
                speed:chart.notes[0].speed,
                pushed: false,
                missed: false,
                judge: JudgeAccuracy::None,
            });
        } else if chart.notes[0].note_type == NoteType::Short {
            let position = Transform::from_translation(Vec3::new(position_x, position_y, 3.));
            commands.spawn_bundle(SpriteBundle {
                texture: material,
                transform: position,
                ..Default::default()
            }).insert(Note {
                note_type: chart.notes[0].note_type.clone(),
                press_key: chart.notes[0].press_key.clone(),
                release_timing: chart.notes[0].release_timing,
                timing: chart.notes[0].timing,
                speed: chart.notes[0].speed,
                pushed: false,
                missed: false,
                judge: JudgeAccuracy::None,
            });
        }
        chart.notes.pop_front();
        
    }
}

pub fn move_note(
    mut query_note: Query<(Entity, &Note, &mut Transform)>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    time: Res<Time>
) {
    let (_entity, music_timer, _dummy) = timer.single();
    for (_entity, note, mut transform) in query_note.iter_mut() {
        if !music_timer.timer.paused() && music_timer.timer.elapsed_secs() > 0. {
            transform.translation.y -= time.delta_seconds() * STANDARD_NOTE_SPEED * note.speed;
        }
    }
    
}

fn despawn_note(
    commands: &mut Commands,
    input_key: Input<KeyCode>,
    key_code: KeyCode,
    mut note: &mut Note,
    time: &Res<Time>,
    music_timer: &MusicTimer,
    mut score: &mut Scoreboard,
    entity: Entity,
    mut transform: &mut Transform,
    mut lane_query: &mut LongNoteTimer,     //타이머 4개를 Lane별로 사용하여 만드는게나을듯?
) -> (bool, JudgeAccuracy) {
    //Judgement : Perfect 0.04167sec (DJMAX V Respect)
    //            Great   0.09000sec
   
    if !music_timer.timer.paused() {
        if input_key.just_pressed(key_code) {
            //println!("current timer: {}", music_timer.timer.elapsed_secs());
            if note.note_type == NoteType::Short {
                if (note.timing as f32 / 1000. + 0.04167 >= music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.04167 <= music_timer.timer.elapsed_secs()) {
                    //println!("note timing : {}", note.timing as f32 / 1000.);
                    commands.entity(entity).despawn();
                    score.perfect += 1;
                    //println!("perfect {}", note.timing as f32 / 1000.);
                    note.judge = JudgeAccuracy::Perfect;
                    return (true, JudgeAccuracy::Perfect);
                } else if (note.timing as f32 / 1000. + 0.09  >= music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.09 <= music_timer.timer.elapsed_secs()) {
                    //println!("note timing : {}", note.timing as f32 / 1000.);
                    commands.entity(entity).despawn();
                    score.great += 1;
                    //println!("great {}", note.timing as f32 / 1000.);
                    note.judge = JudgeAccuracy::Great;
                    return (true, JudgeAccuracy::Great);
                }
            } else if (note.note_type == NoteType::Long) && (note.pushed == false) {
                if (note.timing as f32 / 1000. + 0.04167 >= music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.04167 <= music_timer.timer.elapsed_secs()) {
                    score.perfect += 1;
                    lane_query.timer.set_elapsed(time::Duration::from_secs_f32(0.));
                    note.pushed = true;
                    note.judge = JudgeAccuracy::Perfect;
                    return (true, JudgeAccuracy::Perfect);
                } else if (note.timing as f32 / 1000. + 0.09  >= music_timer.timer.elapsed_secs()) && (note.timing as f32 / 1000. - 0.09 <= music_timer.timer.elapsed_secs()) {
                    score.great += 1;
                    lane_query.timer.set_elapsed(time::Duration::from_secs_f32(0.));
                    note.pushed = true;
                    note.judge = JudgeAccuracy::Great;
                    return (true, JudgeAccuracy::Great);
                } else if note.timing as f32 / 1000. + 0.09 < music_timer.timer.elapsed_secs() {
                    score.miss += 1;
                    note.missed = true;
                    note.judge = JudgeAccuracy::Bad;
                    println!("start miss {}", music_timer.timer.elapsed_secs());
                    return (true, JudgeAccuracy::Miss);
                }
            }
        }
        //아예 안누르면 Short이든, Long이든 삭제. Short의 경우 release_timing과 timing이 동일

        if input_key.just_released(key_code) && note.note_type == NoteType::Long {
            if (note.release_timing as f32 / 1000. + 0.09 >= music_timer.timer.elapsed_secs()) && (note.release_timing as f32 / 1000. - 0.09 <= music_timer.timer.elapsed_secs()) {
                score.perfect += 1;
                commands.entity(entity).despawn();
                return (true, JudgeAccuracy::Perfect);
            } else if (note.missed == false) && (note.timing as f32 / 1000. + 0.09 < music_timer.timer.elapsed_secs()) && (note.release_timing as f32 / 1000. - 0.09 > music_timer.timer.elapsed_secs()){
                note.judge = JudgeAccuracy::Bad;
                score.miss += 1;
                note.missed = true;
                return (true, JudgeAccuracy::Miss);
            }
        }

        if note.note_type == NoteType::Long {
            //계속 누르고 있을때의 동작
            if input_key.pressed(key_code) && (note.timing as f32 / 1000. <= music_timer.timer.elapsed_secs()) && 
                (note.release_timing as f32 / 1000. >= music_timer.timer.elapsed_secs()) {
                //println!("timer: {}", lane_query.timer.elapsed_secs());
                //TODO: 노트가 삭제되는 과정에 대한 개선 필요
                lane_query.timer.tick(time.delta());
                
                let position_diff = time.delta_seconds() * STANDARD_NOTE_SPEED * note.speed;
                transform.translation.y += position_diff / 2.;
                transform.scale.y -= position_diff / 30.;

                if lane_query.timer.just_finished() {
                    return (true, note.judge);
                }
                return (true, JudgeAccuracy::None);
            }
        }
    }
    if note.release_timing as f32 / 1000. + 0.09 < music_timer.timer.elapsed_secs() {
        score.miss += 1;
        println!("end miss or short miss: {}", music_timer.timer.elapsed_secs());
        commands.entity(entity).despawn();
        return (true, JudgeAccuracy::Miss);
    }
    (false, JudgeAccuracy::Miss)
}

//'Z'
pub fn despawn_note_0(
    mut commands: Commands,
    mut query_note: Query<(Entity, &mut Note, &mut Transform)>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    mut score: Query<&mut Scoreboard>,
    mut event_animation: EventWriter<EventAnimation>,
    mut event_key_sound: EventWriter<KeySound1>,
    mut event_combo: EventWriter<EventCombo>,
    mut lane_query: Query<(&mut LongNoteTimer, With<FirstLane>)>
) {
    let (_entity, music_timer, _hold) = timer.single();
    let mut scoreboard = score.single_mut();
    for (entity, mut note, mut transform) in query_note.iter_mut() {
        match note.press_key {
            Press4Key::First => (),
            _ => continue
        };
        let (mut lane, _dummy) = lane_query.single_mut();
        //노트 간격이 좁을 때 한번 누르는 것만으로 간격이 좁은 두 노트가 함께 제거되지 않도록 함
        let (nest, accuracy) = despawn_note(&mut commands, key_input.clone(), KeyCode::Z, &mut note, &time, music_timer, &mut scoreboard, entity, &mut transform, &mut lane);
        if accuracy == JudgeAccuracy::None { return; }
        if nest == true { 
            event_animation.send(EventAnimation {judge: accuracy});
            event_combo.send( EventCombo{ judge: accuracy });
            if accuracy != JudgeAccuracy::Miss {
                event_key_sound.send(KeySound1);
            }
            return;
        }
    }
}

//'X'
pub fn despawn_note_1(
    mut commands: Commands,
    mut query_note: Query<(Entity, &mut Note, &mut Transform)>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    mut score: Query<&mut Scoreboard>,
    mut event_animation: EventWriter<EventAnimation>,
    mut event_key_sound: EventWriter<KeySound2>,
    mut event_combo: EventWriter<EventCombo>,
    mut lane_query: Query<(&mut LongNoteTimer, With<SecondLane>)>,
) {
    let (_entity, music_timer, _hold) = timer.single();
    let mut scoreboard = score.single_mut();
    for (entity, mut note, mut transform) in query_note.iter_mut() {
        match note.press_key {
            Press4Key::Second => (),
            _ => continue
        };
        //노트 간격이 좁을 때 한번 누르는 것만으로 간격이 좁은 두 노트가 함께 제거되지 않도록 함
        let (mut lane, _dummy) = lane_query.single_mut();
        let (nest, accuracy) = despawn_note(&mut commands, key_input.clone(), KeyCode::X, &mut note, &time ,music_timer, &mut scoreboard, entity, &mut transform, &mut lane);
        if accuracy == JudgeAccuracy::None { return; }
        if nest == true { 
            event_animation.send(EventAnimation {judge: accuracy});
            event_combo.send( EventCombo{ judge: accuracy });
            if accuracy != JudgeAccuracy::Miss {
                event_key_sound.send(KeySound2);
            }
            return;
        }
    }
}

//'.'
pub fn despawn_note_2(
    mut commands: Commands,
    mut query_note: Query<(Entity, &mut Note, &mut Transform)>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    mut score: Query<&mut Scoreboard>,
    mut event_animation: EventWriter<EventAnimation>,
    mut event_key_sound: EventWriter<KeySound3>,
    mut event_combo: EventWriter<EventCombo>,
    mut lane_query: Query<(&mut LongNoteTimer, With<ThirdLane>)>,
) {
    let (_entity, music_timer, _hold) = timer.single();
    let mut scoreboard = score.single_mut();
    for (entity, mut note, mut transform) in query_note.iter_mut() {
        match note.press_key {
            Press4Key::Third => (),
            _ => continue
        };
        //노트 간격이 좁을 때 한번 누르는 것만으로 간격이 좁은 두 노트가 함께 제거되지 않도록 함
        let (mut lane, _dummy) = lane_query.single_mut();
        let (nest, accuracy) = despawn_note(&mut commands, key_input.clone(), KeyCode::Period, &mut note, &time, music_timer, &mut scoreboard, entity, &mut transform, &mut lane);
        if accuracy == JudgeAccuracy::None { return; }
        if nest == true {
            event_animation.send(EventAnimation {judge: accuracy});
            event_combo.send( EventCombo{ judge: accuracy });
            if accuracy != JudgeAccuracy::Miss {
                event_key_sound.send(KeySound3);
            }
            return; 
        }
    }
}

//'/'
pub fn despawn_note_3(
    mut commands: Commands,
    mut query_note: Query<(Entity, &mut Note, &mut Transform)>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>,
    mut score: Query<&mut Scoreboard>,
    mut event_animation: EventWriter<EventAnimation>,
    mut event_key_sound: EventWriter<KeySound4>,
    mut event_combo: EventWriter<EventCombo>,
    mut lane_query: Query<(&mut LongNoteTimer, With<FourthLane>)>,
) {
    let (_entity, music_timer, _hold) = timer.single();
    let mut scoreboard = score.single_mut();
    for (entity, mut note, mut transform) in query_note.iter_mut() {
        match note.press_key {
            Press4Key::Fourth => (),
            _ => continue
        };
        //노트 간격이 좁을 때 한번 누르는 것만으로 간격이 좁은 두 노트가 함께 제거되지 않도록 함
        let (mut lane, _dummy) = lane_query.single_mut();
        let (nest, accuracy) = despawn_note(&mut commands, key_input.clone(), KeyCode::Slash, &mut note, &time, music_timer, &mut scoreboard, entity, &mut transform, &mut lane);
        if accuracy == JudgeAccuracy::None { return; }
        if nest == true {
            event_animation.send(EventAnimation {judge: accuracy});
            event_combo.send( EventCombo{ judge: accuracy });
            if accuracy != JudgeAccuracy::Miss {
                event_key_sound.send(KeySound4);
            }
            return; 
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

#[derive(PartialEq, Copy, Clone)]
pub enum JudgeAccuracy {
    Perfect = 100,
    Great = 90,
    Bad = 10,
    Miss = 0,
    None,
}

#[derive(Component)]
pub struct JudgeTimer(Timer);
#[derive(Component)]
pub struct Scale(f32);


//x:0, y:-50
pub fn spawn_judgement(
    mut commands: Commands,
    mut events: EventReader<EventAnimation>,
    materials: Res<JudgeResource>,
) {
    let mut judge_transform = Transform::from_translation(Vec3::new(0., -50., 4.));
    for accuracy in events.iter() {
        match accuracy {
            EventAnimation {judge : JudgeAccuracy::Perfect} => {
                let timer = JudgeTimer(Timer::from_seconds(1.5, false));
                let scale = Scale(1.0);
                judge_transform.scale = Vec3::splat(scale.0);
                commands.spawn_bundle( SpriteBundle {
                    texture: materials.perfect.clone(),
                    transform: judge_transform,
                    ..Default::default()
                }).insert(timer).insert(scale);
                return;
            },
    
            EventAnimation {judge : JudgeAccuracy::Great} => {
                let timer = JudgeTimer(Timer::from_seconds(1.5, false));
                let scale = Scale(1.0);
                judge_transform.scale = Vec3::splat(scale.0);
                commands.spawn_bundle( SpriteBundle {
                    texture: materials.great.clone(),
                    transform: judge_transform,
                    ..Default::default()
                }).insert(timer).insert(scale);
                return;
            },

            EventAnimation {judge : JudgeAccuracy::Bad} => {
                let timer = JudgeTimer(Timer::from_seconds(1.5, false));
                let scale = Scale(1.0);
                judge_transform.scale = Vec3::splat(scale.0);
                commands.spawn_bundle( SpriteBundle {
                    texture: materials.bad.clone(),
                    transform: judge_transform,
                    ..Default::default()
                }).insert(timer).insert(scale);
                return;
            },
    
            EventAnimation {judge : JudgeAccuracy::Miss} => {
                let timer = JudgeTimer(Timer::from_seconds(1.5, false));
                let scale = Scale(1.0);
                judge_transform.scale = Vec3::splat(scale.0);
                commands.spawn_bundle( SpriteBundle {
                    texture: materials.miss.clone(),
                    transform: judge_transform,
                    ..Default::default()
                }).insert(timer).insert(scale);
                return;
            },

            _ => return
        }
    }
}

pub fn update_judgement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Scale, &mut JudgeTimer)>
) {
    let mut query_number = 0;
    for(_dummy, _dummy1, _dummy2, _dummy3) in query.iter_mut() {
        query_number += 1;
    }

    for (entity, mut transform, mut scale, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.elapsed_secs() < 0.1 && scale.0 > 0.7 {
            transform.scale = Vec3::splat(scale.0);
            scale.0 -= 0.02;
        }
        if timer.0.finished() || query_number >= 2 {
            commands.entity(entity).despawn();
            query_number -= 1;
        }
    }
}


pub fn setup_background_text(
    mut commands: Commands,
    font_resource: Res<FontResource>,
) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
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
        ..default()
    })
    .insert(TimerText);

    commands.spawn_bundle(TextBundle {
        node: Node {size: Vec2::new(500., 100.)},
        style: Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::FlexEnd,
            position: UiRect::<bevy::ui::Val> { top: Val::Px(30.), ..Default::default() },
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
                    value: "\nBad : ".to_string(),
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
                },
            ],
            ..default()
        },
        //transform: Transform::from_translation(Vec3::new(-350., 450., 10.)),
        ..default()
    }).insert(Scoreboard {perfect:0, great:0, miss:0, bad:0,});
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
        score_text.sections[5].value = scoreboard.bad.to_string();
        score_text.sections[7].value = scoreboard.miss.to_string();
}

pub fn setup_accuracy(
    mut commands: Commands,
    font_resource: Res<FontResource>
) {
    commands.spawn_bundle(Text2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
        text: Text::from_section(
            "0.0%",
            TextStyle {
                font: font_resource.font.clone(),
                font_size: 25.,
                color: Color::rgba(0.98, 0.92, 0.92, 0.5)
            }
        ).with_alignment(TextAlignment::CENTER),
        ..Default::default()
        }).insert(AccuracyText);
        commands.spawn().insert(TotalAccuracy(0.));
}

pub fn update_accuracy(
    mut text_query: Query<(&mut Text, &AccuracyText)>,
    mut accuracy_query: Query<&mut TotalAccuracy>,
    scoreboard_query: Query<&Scoreboard>
) {
    let mut accuracy = accuracy_query.single_mut();
    let scoreboard = scoreboard_query.single();
    let total_score = 100 * (scoreboard.perfect + scoreboard.great + scoreboard.miss);
    let total;
    if total_score != 0 {
        total = (scoreboard.perfect * 100 + scoreboard.great * 90) as f32 / total_score as f32;
    } else {return;}
    accuracy.0 = total * 100.;
    let total_text = format!("{:0.02}%", accuracy.0);
    
    let (mut text, _dummy) = text_query.single_mut();
    text.sections[0].value = total_text;
}

pub fn setup_combo(
    mut commands: Commands,
    font_resource: Res<FontResource>
) {
    commands.spawn_bundle(Text2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 250., 4.)),
        text: Text::from_section(
            "0",
            TextStyle {
                font: font_resource.font.clone(),
                font_size: 50.,
                color: Color::rgba(0.98, 0.92, 0.92, 0.5)
            }
        ).with_alignment(TextAlignment::CENTER),
        ..Default::default()
        }).insert(ComboText);
    commands.spawn().insert(Combo(0));
}

//차후에 ComboResource로 그림수정
/*
pub fn spawn_combo_effect(
    mut commands: Commands,
    mut event_combo: EventReader<EventCombo>,
    query: Query<(&Combo)>
) {

}
*/

pub fn update_combo_effect(
    mut combo_query: Query<&mut Combo>,
    mut text_query: Query<(&mut Text, &ComboText)>,
    mut event_combo: EventReader<EventCombo>
) {
    let mut combo = combo_query.single_mut();
    for event in event_combo.iter() {
        match event.judge {
            JudgeAccuracy::Miss => {combo.0 = 0},
            _ => {combo.0 += 1},
        }
    }
    let combo_text = combo.0;

    let (mut text, _dummy) = text_query.single_mut();
    text.sections[0].value = combo_text.to_string();
}




//for debug
pub fn _show_playing_timer(
    timer: Query<(Entity, &MusicTimer, Without<Hold>)>
) {
    for (_entity, music_timer, _dummy) in timer.iter() {
        println!("{}", music_timer.timer.elapsed_secs());  
    }
}

pub fn pause_game(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mut timer: Query<(Entity, &mut MusicTimer)>,
    text: Query<(Entity, &PausedText)>,
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
            for (entity, _paused_text) in text.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn print_keyboard_event_system(
    mut keyboard_input_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    music_timer: Query<&MusicTimer>,
) {
    for event in keyboard_input_events.iter() {
        for timer in music_timer.iter() {
            println!("Key Time: {}", timer.timer.elapsed_secs());
        }
        info!("{:?}", event);
    }
}

pub fn open_chart(mut commands: Commands) {
    let chart_file = File::open("assets/music/PUPA/PUPA.txt").expect("file not found");

    //Note를 Spawn하거나 Despawn할 때 한번에 4개를 동시에 처리할 수 있도록 저장하는 Stack을 나눔
    let mut chart_vec_0: VecDeque<Note> = VecDeque::new();
    let mut chart_vec_1: VecDeque<Note> = VecDeque::new();
    let mut chart_vec_2: VecDeque<Note> = VecDeque::new();
    let mut chart_vec_3: VecDeque<Note> = VecDeque::new();

    let mut buffer = BufReader::new(chart_file);
    let mut line: String = String::new();

    loop {
        let read_bytes = buffer.read_line(&mut line).unwrap();
        //println!("Buffer: {}", line.trim());
        //println!("Read Bytes: {}", read_bytes);
        if read_bytes == 0 {
            break;
        }
        
        let parsed_note = parse_file_string(&line).unwrap();
        match parsed_note.press_key {
            Press4Key::First => {
                chart_vec_0.push_back(parsed_note);
            },
            Press4Key::Second => {
                chart_vec_1.push_back(parsed_note);
            },
            Press4Key::Third => {
                chart_vec_2.push_back(parsed_note);
            },
            Press4Key::Fourth => { 
                chart_vec_3.push_back(parsed_note);
            }
        }
        line.clear();
    }

    //먼저 눌러야하는 순으로 정렬하여 나중에 spawn_note system에서 사용이 더 용이하도록 함
    chart_vec_0.make_contiguous().sort_by(|a, b| a.timing.cmp(&b.timing));
    chart_vec_1.make_contiguous().sort_by(|a, b| a.timing.cmp(&b.timing));
    chart_vec_2.make_contiguous().sort_by(|a, b| a.timing.cmp(&b.timing));
    chart_vec_3.make_contiguous().sort_by(|a, b| a.timing.cmp(&b.timing));

    let chart_0 = Chart {
        notes: chart_vec_0,
    };
    let chart_1 = Chart {
        notes: chart_vec_1,
    };
    let chart_2 = Chart {
        notes: chart_vec_2,
    };
    let chart_3 = Chart {
        notes: chart_vec_3,
    };

    //Resource가 아닌 Entity로써 Chart를 관리하여 수정, 삭제를 용이하게 함
    commands.spawn().insert(chart_0).insert(FirstLane);
    commands.spawn().insert(chart_1).insert(SecondLane);
    commands.spawn().insert(chart_2).insert(ThirdLane);
    commands.spawn().insert(chart_3).insert(FourthLane);

    //Music은 최대 MAX_MUSIC_LENGTH / 1000 만큼의 길이를 가짐
    let music_timer = MusicTimer {timer: Timer::from_seconds(MAX_MUSIC_LENGTH / 1000., false)};
    commands.spawn().insert(music_timer);

    //게임시작하고 HOLD_TIME / 1000만큼 대기
    let hold_timer = MusicTimer { timer: Timer::from_seconds(HOLD_TIME / 1000., false)}; 
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
        release_timing: 0,
        speed: 17.4, // 6.4? = 17.4?
        pushed: false,
        missed: false,
        judge: JudgeAccuracy::None,
    };
    //println!("parsed string: {}", string.trim());
    for c in string.chars() {
        if c == ',' {
            match commas {
                0 => { 
                    let parsed: usize = (&string[start..end]).parse().unwrap();
                    start = end + 1;
                    //println!("commas 0: {}", parsed);
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
                    //println!("commas 1: {}", parsed);
                    match parsed {
                        "Short" => note.note_type = NoteType::Short,
                        "Long" => note.note_type = NoteType::Long,
                        _ => panic!("parsing type error")
                    }
                }

                2 => {
                    let parsed: usize = (&string[start..end]).parse().unwrap();
                    start = end + 1;
                    //println!("commas 2: {}\n", parsed);
                    note.timing = parsed;
                    note.release_timing = parsed;
                }
                3 => {
                    if note.note_type == NoteType::Short {
                      break;
                    }
                    let parsed: usize = (&string[start..end]).parse().unwrap();
                    note.release_timing = parsed;
                }
                4 => {
                    break;
                }
                _ => panic!("parsing comma error : {}", commas),
            }
            commas = commas + 1;
        } 
        end = end + 1;
    }

    if commas == 3 || commas == 4 {
        Ok(note)
    } else {
        Err("Not parsed")
    }
}