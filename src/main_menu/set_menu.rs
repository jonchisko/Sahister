use bevy::prelude::*;
use crate::app_states::AppState;
use super::panel_builder::PanelBuilder;
use super::button_events::MainMenuEvent;
use crate::{SkinSetResource, ChessboardType, FiguresType};
use super::button_builder::{self, ButtonBuilder, ButtonType, ClassicButton};

pub struct SetMenuPlugin;

impl Plugin for SetMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::SetMenu)
                .with_system(setup_set_menu)
            )
            .add_system_set(
                SystemSet::on_update(AppState::SetMenu)
                .with_system(handle_set_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::SetMenu)
                .with_system(close_set_menu)
            );
    }
}

#[derive(Component, PartialEq)]
struct SetButton {
    button_type: SetButtonType,
    selected: bool,
}

impl ClassicButton for SetButton {
    fn get_button_name(&self) -> String {
        String::from("SET BUTTON")
    }

    fn get_button_type(&self) -> Box<dyn ButtonType> {
        Box::new(self.button_type)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SetButtonType {
    SetNormalChessboard,
    SetWoodChessboard,
    SetBwSet,
    SetNormalSet,
    SetRbSet,
    BackToMainMenu,
}

impl ButtonType for SetButtonType {
    fn get_type_name(&self) -> String {
        match self {
            SetButtonType::SetNormalChessboard => {String::from("NORMAL")},
            SetButtonType::SetWoodChessboard => {String::from("WOODEN")},
            SetButtonType::SetBwSet => {String::from("BLACK 'n WHITE")},
            SetButtonType::SetNormalSet => {String::from("NORMAL")},
            SetButtonType::SetRbSet => {String::from("RED 'n BLUE")},
            SetButtonType::BackToMainMenu => {String::from("MAIN MENU")},
        }
    }
}

fn setup_set_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {

        PanelBuilder::build_vertical_panel::<SetButton>(
                parent,
                300.0,
                "CHESSBOARD SET",
                &asset_server,
                vec![
                    SetButton {
                        button_type: SetButtonType::SetNormalChessboard,
                        selected: true
                    },
                    SetButton {
                        button_type: SetButtonType::SetWoodChessboard,
                        selected: false
                    }
                ],
                ButtonBuilder::build_button,
        );

        PanelBuilder::build_vertical_panel::<SetButton>(
            parent,
            300.0,
            "FIGURE SET",
            &asset_server,
            vec![
                SetButton {
                    button_type: SetButtonType::SetNormalSet,
                    selected: true
                },
                SetButton {
                    button_type: SetButtonType::SetBwSet,
                    selected: false
                },
                SetButton {
                    button_type: SetButtonType::SetRbSet,
                    selected: false
                },
                SetButton {
                    button_type: SetButtonType::BackToMainMenu,
                    selected: false
                }
            ],
            ButtonBuilder::build_button,
        );
    });
}

fn handle_set_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &SetButton),
        (Changed<Interaction>, With<Button>)>,
    mut skin_res: ResMut<SkinSetResource>,
    mut event_writer: EventWriter<MainMenuEvent>,
) {

    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor(button_builder::PRESSED_BUTTON);
                match button.button_type {
                    SetButtonType::SetNormalChessboard => {
                        skin_res.selected_chessboard = Some(ChessboardType::Normal);
                    },
                    SetButtonType::SetWoodChessboard => {
                        skin_res.selected_chessboard = Some(ChessboardType::Wooden);
                    },
                    SetButtonType::SetNormalSet => {
                        skin_res.selected_figures = Some(FiguresType::Normal);
                    },
                    SetButtonType::SetBwSet => {
                        skin_res.selected_figures = Some(FiguresType::Bw);
                    },
                    SetButtonType::SetRbSet => {
                        skin_res.selected_figures = Some(FiguresType::Rb);
                    },
                    SetButtonType::BackToMainMenu => {
                        event_writer.send(MainMenuEvent);
                    }
                }
            },
            Interaction::Hovered => {
                *color = UiColor(button_builder::HOVERED_BUTTON);
            },
            Interaction::None => {
                *color = UiColor(button_builder::NORMAL_BUTTON);
            },
        }
    }
}

fn close_set_menu(
    mut commands: Commands,
    set_menu_buttons: Query<Entity, With<Node>>
) {
    for entity_id in set_menu_buttons.iter() {
        commands.entity(entity_id).despawn();
    }
}