use bevy::prelude::*;

pub const CAMERA_SCALE: f32 = 0.33;
const CAMERA_MOVE_SPEED: f32 = 3.0f32;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(camera_mover);
    }
}

#[derive(Default)]
struct PrevCursorPos {
    pos: Option<Vec2>,
}

#[derive(Component)]
pub struct MainCamera;


fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = CAMERA_SCALE;

    commands
        .spawn_bundle(camera)
        .insert(MainCamera);

    commands.insert_resource(PrevCursorPos {
        pos: None,
    });
}

fn camera_mover(
    mut prev_cursor_pos: ResMut<PrevCursorPos>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let cursor_pos = windows.get_primary().expect("Could not get primary window").cursor_position();
    if mouse_input.pressed(MouseButton::Right) && cursor_pos.is_some() {
        let camera_move: Option<Vec2> = match prev_cursor_pos.pos {
            Some(prev_val) => {
                Some((prev_val - cursor_pos.unwrap()).normalize_or_zero())
            },
            None => {
                None
            }
        };

        match camera_move {
            Some(dir) => {
                let mut camera = camera.single_mut();
                let cur_translation = camera.translation;
                camera.translation = Vec3::new(cur_translation.x + dir.x * CAMERA_MOVE_SPEED, cur_translation.y + dir.y * CAMERA_MOVE_SPEED, 0.0);
            },
            None => {}
        }
    }

    prev_cursor_pos.pos = cursor_pos;
}