use bevy::prelude::*;
use crate::app_states::AppState;

mod set_menu;
mod credits_menu;
mod button_events;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::MainMenu)
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu)
            );
    }
}

fn setup_menu() {
    todo!()
}

fn close_menu() {
    todo!()
}

fn handle_buttons() {
    todo!()
}