use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::state::GameState;
use crate::notes::FontResource;
pub struct SelectMenuPlugin;

impl Plugin for SelectMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuResource>()
            .add_system_set(
                SystemSet::on_enter(GameState::SelectMenu)
                .with_system(setup_menu)
            );
    }
}

pub struct SongInfo {
    name: String,
    time: f32,
    difficult: f32,
}

pub struct MenuResource {
    music_button: Handle<Image>,
}

impl FromWorld for MenuResource {
    fn from_world(world: &mut World)-> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let menu_resource = MenuResource {
            music_button: asset_server.load("image/select_menu/music.png")
        };
        
        menu_resource
    }
}

pub fn setup_menu(
    mut commands: Commands,
    font_resource: Res<FontResource>,
    button_resource: Res<MenuResource>
) {
    commands.spawn_bundle( ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content : JustifyContent::Center,
            //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle( ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: button_resource.music_button.clone().into(),
            ..Default::default()
        }).insert(FocusPolicy::Pass).with_children(|parent| {
            parent.spawn_bundle( TextBundle {
                text: Text::from_section(
                    "PUPA",
                    TextStyle {
                        font: font_resource.font.clone(),
                        font_size: 30.0,
                        color: Color::rgba(0.9, 0.9, 0.9, 1.)
                    }),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });
    });
}

pub fn music_button_interaction(
    mut interaction_query: Query<(&Children, &Interaction),Changed<Interaction>>
) {

}