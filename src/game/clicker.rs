use bevy::prelude::*;
use super::figures::{ChessTile, Figure};
use crate::camera_controller::{self, MainCamera};

struct ClickerGamePlugin;

impl Plugin for ClickerGamePlugin {
    fn build(&self, app: &mut App) {

    }
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
    selections: ResMut<TheTwoSelections>,
    camera: Query<&Transform, With<MainCamera>>,
) {
    let window = windows.get_primary().expect("No primary window");
    if let Some(mouse_screen_pos) = window.cursor_position() {
        
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