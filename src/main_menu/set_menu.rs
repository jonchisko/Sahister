use bevy::prelude::*;
use crate::app_states::AppState;
use super::panel_builder::PanelBuilder;
use super::button_builder::ButtonBuilder;
use super::button_builder::{ButtonType, ClassicButton};

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

#[derive(Component)]
struct SetButton {
    button_type: SetButtonType,
}

impl ClassicButton for SetButton {
    fn get_button_name(&self) -> String {
        String::from("SET BUTTON")
    }

    fn get_button_type(&self) -> Box<dyn ButtonType> {
        Box::new(self.button_type)
    }
}

#[derive(Clone, Copy)]
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
            SetButtonType::SetNormalSet => {String::from("RED 'n BLUE")},
            SetButtonType::SetRbSet => {String::from("NORMAL")},
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
                        button_type: SetButtonType::SetNormalChessboard
                    },
                    SetButton {
                        button_type: SetButtonType::SetWoodChessboard
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
                    button_type: SetButtonType::SetNormalSet
                },
                SetButton {
                    button_type: SetButtonType::SetBwSet
                },
                SetButton {
                    button_type: SetButtonType::SetRbSet
                },
                SetButton {
                    button_type: SetButtonType::BackToMainMenu
                }
            ],
            ButtonBuilder::build_button,
        );
    });
}

fn handle_set_buttons(

) {

}

fn close_set_menu(

) {

}