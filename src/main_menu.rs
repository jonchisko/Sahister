use bevy::prelude::*;
use bevy::app::AppExit;
use set_menu::SetMenuPlugin;
use crate::{app_states::AppState, logger};
use button_builder::{ButtonType, ClassicButton, ButtonBuilder};
use button_events::{StartGameEvent, SetMenuEvent};

use self::button_events::MainMenuEvent;

mod set_menu;
mod button_events;
mod button_builder;
mod panel_builder;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::MainMenu)
            .add_event::<StartGameEvent>()
            .add_event::<SetMenuEvent>()
            .add_event::<MainMenuEvent>()
            .add_system(handle_back_to_main_menu_transition)
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_menu_buttons)
                .with_system(handle_set_menu_transition)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu)
            )
            .add_plugin(SetMenuPlugin);
    }
}

#[derive(Component)]
struct MenuButton {
    button_type: MenuButtonType,
}

impl ClassicButton for MenuButton {
    fn get_button_name(&self) -> String {
        String::from("MENU BUTTON")
    }

    fn get_button_type(&self) -> Box<dyn ButtonType> {
        Box::new(self.button_type)
    }
}

#[derive(Clone, Copy)]
enum MenuButtonType {
    PlayButton,
    SkinSetsButton,
    QuitButton,
}

impl ButtonType for MenuButtonType {
    fn get_type_name(&self) -> String {
        match self {
            MenuButtonType::PlayButton => {String::from("PLAY")},
            MenuButtonType::SkinSetsButton => {String::from("SKIN SETS")},
            MenuButtonType::QuitButton => {String::from("QUIT")},
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(33.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            image: UiImage(asset_server.load("./sprites/ui/background.png")),
            ..Default::default()
        })
        .with_children(|parent| {

            ButtonBuilder::build_button(
                parent, 
                MenuButton {
                    button_type: MenuButtonType::PlayButton,
                    },
                    &asset_server,    
            );

            ButtonBuilder::build_button(
                parent, 
                MenuButton {
                    button_type: MenuButtonType::SkinSetsButton,
                    },
                    &asset_server,    
            );

            ButtonBuilder::build_button(
                parent, 
                MenuButton {
                    button_type: MenuButtonType::QuitButton,
                    },
                    &asset_server,    
            );
        });
    });
}

fn close_menu(
    mut commands: Commands,
    main_menu_buttons: Query<Entity, With<Node>>
) {
    for entity_id in main_menu_buttons.iter() {
        commands.entity(entity_id).despawn();
    }
}

fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuButton),
        (Changed<Interaction>, With<Button>)>,
    mut start_event: EventWriter<StartGameEvent>,
    mut set_event: EventWriter<SetMenuEvent>,
    mut exit_event: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = button_builder::PRESSED_BUTTON.into();
                match button.button_type {
                    MenuButtonType::PlayButton => {
                        start_event.send(StartGameEvent);
                    },
                    MenuButtonType::SkinSetsButton => {
                        set_event.send(SetMenuEvent);
                    },
                    MenuButtonType::QuitButton => {
                        exit_event.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_builder::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = button_builder::NORMAL_BUTTON.into();
            }
        }
    }
}

fn handle_set_menu_transition(
    mut app_state: ResMut<State<AppState>>,
    mut event_reader: EventReader<SetMenuEvent>,
) {
    if event_reader.iter().next().is_some() {
        match app_state.set(AppState::SetMenu) {
            Ok(_) => {},
            Err(msg) => {
                logger::log(msg);
            }
        }
    }
}

fn handle_back_to_main_menu_transition(
    mut app_state: ResMut<State<AppState>>,
    mut event_reader: EventReader<MainMenuEvent>,
) {
    if event_reader.iter().next().is_some() {
        match app_state.set(AppState::MainMenu) {
            Ok(_) => {},
            Err(msg) => {
                logger::log(msg);
            }
        }
    }
}