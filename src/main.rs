use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod notes;

fn main() {
    let mut app = App::new();    
        app.insert_resource(Msaa { samples: 4});
        app.insert_resource(WindowDescriptor {
            title: "rhythme 0.1.0".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        });
   //#[cfg(target_arch = "wasm32")]
    //app.add_plugins(bevy_webgl2::DefaultPlugins);
    app.add_plugins(DefaultPlugins);
    app.add_plugin(AudioPlugin);
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(bevy_framepace::FramepacePlugin::default());
    app.add_startup_system(camera_setup);
    app.add_startup_system(frame_limit);
   // app.add_system(exit_on_esc_system);
   // app.add_startup_system(notes::playing_audio);
    app.add_plugin(notes::NotePlugin);
    //app.add_system(notes::print_keyboard_event_system); // for debug
    //app.add_plugin(FrameTimeDiagnosticsPlugin::default()); // for debug
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    //Needs for see UI
    commands.spawn_bundle(UiCameraBundle::default());
}

fn frame_limit(
    mut setting: ResMut<bevy_framepace::FramepacePlugin>
) {
    setting.framerate_limit = bevy_framepace::FramerateLimit::Manual(60);
}