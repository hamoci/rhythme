use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use notes::NotePlugin;

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
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(bevy_framepace::FramepacePlugin::default());
    app.add_startup_system(camera_setup);
    app.add_startup_system(frame_limit);
    app.add_system(exit_on_esc_system);
   // app.add_startup_system(notes::playing_audio);
    app.add_plugin(notes::NotePlugin);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn frame_limit(
    mut setting: ResMut<bevy_framepace::FramepacePlugin>
) {
    setting.framerate_limit = bevy_framepace::FramerateLimit::Manual(60);
}