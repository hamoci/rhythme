use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

mod notes;

fn main() {
    let mut app = App::new();    
        app.insert_resource(Msaa { samples: 4});
        app.insert_resource(WindowDescriptor {
            title: "rhythme 0.1.0".to_string(),
            width: 1000.0,
            height: 800.0,
            ..Default::default()
        });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(camera_setup);
    app.add_system(exit_on_esc_system);
    app.add_startup_system(notes::playing_audio);
    app.add_startup_system(notes::load_note_asset);
    app.add_startup_system(notes::open_chart);
    app.add_startup_system(notes::playing_setup);
    app.add_system(notes::spawn_note);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}