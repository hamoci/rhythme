use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::state::GameState;
use crate::notes::FontResource;
pub struct SelectMenuPlugin;

#[derive(Component)]
pub struct MusicList;

impl Plugin for SelectMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuResource>()
            .add_system_set(
                SystemSet::on_enter(GameState::SelectMenu)
                .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_resume(GameState::SelectMenu)
                .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(GameState::SelectMenu)
                .with_system(music_button_interaction)
            )
            .add_system_set(
                SystemSet::on_pause(GameState::SelectMenu)
                .with_system(despawn_music_button)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::SelectMenu)
                .with_system(despawn_music_button)
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
    music_hover: Handle<Image>,
    music_clicked: Handle<Image>
}

impl FromWorld for MenuResource {
    fn from_world(world: &mut World)-> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let menu_resource = MenuResource {
            music_button: asset_server.load("image/select_menu/music.png"),
            music_hover: asset_server.load("image/select_menu/music_hover.png"),
            music_clicked: asset_server.load("image/select_menu/music_clicked.png")
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
            align_self: AlignSelf::FlexEnd,
            align_items: AlignItems::FlexEnd,
            justify_content : JustifyContent::Center,
            position: UiRect::new(Val::Px(250.), Val::Auto, Val::Auto, Val::Auto),
            size: Size::new(Val::Px(500.0), Val::Px(60.0)),
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
                        font_size: 30.,
                        color: Color::rgba(0.9, 0.9, 0.9, 1.)
                    }),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });
    }).insert(MusicList);
}

pub fn music_button_interaction(
    interaction_query: Query<(&Children, &Interaction), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut image_query: Query<&mut UiImage>,
    button_resource: Res<MenuResource>
) {
    for (children, interaction) in interaction_query.iter() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();
        match interaction {
            Interaction::Clicked => {
                *image = UiImage(button_resource.music_clicked.clone());
                state.push(GameState::InGame).unwrap();
            },
            Interaction::Hovered => {
                *image = UiImage(button_resource.music_hover.clone());
            },
            Interaction::None => {
                *image = UiImage(button_resource.music_button.clone());
            }
        }
    }
}

pub fn despawn_music_button(
    mut commands: Commands,
    button_query: Query<(Entity, &Button)>
) {
    for (entity, button) in button_query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}