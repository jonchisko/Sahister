use bevy::prelude::*;
use crate::app_states::AppState;

pub struct CreditsMenuPlugin;

impl Plugin for CreditsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::CreditsMenu)
                .with_system(setup_credits_menu)
            )
            .add_system_set(
                SystemSet::on_update(AppState::CreditsMenu)
                .with_system(handle_credits_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::CreditsMenu)
                .with_system(close_credits_menu)
            );
    }
}

fn setup_credits_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

}

fn handle_credits_buttons(

) {

}

fn close_credits_menu(

) {
    
}