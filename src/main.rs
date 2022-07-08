/*use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
mod screens;

const RENDER_CYCLE: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "rhythme 0.1.0".to_string(),
            width: 1200.,
            height: 800.,
            present_mode: bevy::window::PresentMode::Mailbox,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(screens::ui_select_music)
        .run();
}
*/

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
    app.add_startup_system(camera_setup)
        .add_system(exit_on_esc_system)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}