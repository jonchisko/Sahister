use bevy::prelude::*;
use super::figures::{ChessTile, Figure, ChessColor};
use crate::camera_controller::{self, MainCamera};

struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
    fn build(&self, app: &mut App) {

    }
}


#[derive(Default)]
struct CurrentPlayer {
    current_color: Option<ChessColor>,
}

#[derive(Default)]
struct TheTwoSelections {
    selection1: Option<(ChessTile, Option<Figure>)>,
    selection2: Option<(ChessTile, Option<Figure>)>,
}

impl TheTwoSelections {
    fn new() -> TheTwoSelections {
        TheTwoSelections {
            selection1: None,
            selection2: None,
        }
    }
}

fn click_tile(
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
    mut selections: ResMut<TheTwoSelections>,
    camera: Query<&Transform, With<MainCamera>>,
    tiles: Query<(&Transform, &ChessTile)>,
    figures: Query<&Figure>,
    current_player: Res<CurrentPlayer>,
) {
    if input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("No primary window");
        let camera_transform = camera.single();
        if let Some(mouse_screen_pos) = window.cursor_position() {
            let mouse_world_pos= get_mouse_world(mouse_screen_pos, camera_transform, window);
            
            let mut clicked_tile = None;
            for (tile_transform, tile) in tiles.iter() {
                if is_mouse_overlapping_tile(tile_transform, &mouse_world_pos) {
                    clicked_tile = Some(tile);
                }
            }

            if clicked_tile.is_some() {
                let figure = get_figure_on_tile(clicked_tile.unwrap(), figures);
                set_selections(
                    &mut selections,
                    *clicked_tile.unwrap(),
                    figure,
                    current_player
                );
            } else {
                reset_selection(&mut selections);
            }
        }
    }
}

// copied from game of life
fn get_mouse_world(pos: Vec2, main_transform: &Transform, window: &Window) -> Vec3 {
    let center = main_transform.translation.truncate();
    let half_width = (window.width() / 2.0) * camera_controller::CAMERA_SCALE;
    let half_height = (window.height() / 2.0) * camera_controller::CAMERA_SCALE;
    let left = center.x - half_width;
    let bottom = center.y - half_height;

    Vec3::new(
        left + pos.x * camera_controller::CAMERA_SCALE,
        bottom + pos.y * camera_controller::CAMERA_SCALE,
        0.0,
    )
}

fn is_mouse_overlapping_tile(tile: &Transform, mouse_pos: &Vec3) -> bool {
    let tile_pos = tile.translation;
    let half_dim = super::TILE_DIM as f32 / 2.0;
    return mouse_pos.x >= tile_pos.x - half_dim && mouse_pos.x < tile_pos.x + half_dim 
        && mouse_pos.y >= tile_pos.y - half_dim && mouse_pos.y < tile_pos.y + half_dim;
}

fn get_figure_on_tile(tile: &ChessTile, figures: Query<&Figure>) -> Option<Figure> {
    for fig in figures.iter() {
        if tile.col == fig.col && tile.row == fig.row {
            return Some(*fig);
        }
    }
    None
}

fn set_selections(
    selections: &mut ResMut<TheTwoSelections>, 
    clicked_tile: ChessTile,
    figure: Option<Figure>,
    current_player: Res<CurrentPlayer>,
) {
    if selections.selection1.is_none() {
        if let Some(fig) = figure {
            if fig.color == current_player.current_color.expect("Current player is missing current color") {
                selections.selection1 = Some((clicked_tile, figure));
            }
        }
    } else {
        selections.selection2 = Some((clicked_tile, figure));
    }
}

fn reset_selection(selections: &mut ResMut<TheTwoSelections>) {
    selections.selection1 = None;
    selections.selection2 = None;
}