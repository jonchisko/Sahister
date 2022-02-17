use bevy::prelude::*;
use crate::app_states::AppState;


pub struct SetMenuPlugin;

impl Plugin for SetMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::SetMenu)
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

fn setup_set_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

}

fn handle_set_buttons(

) {

}

fn close_set_menu(

) {

}