use bevy::prelude::*;

const GRID: u8 = 8;
const TILE_DIM: u8 = 32;
const NORMAL_MAP_TILE_PATH: &str = "./sprites/map/normal_set/";
const CAMERA_MOVE_SPEED: f32 = 3.0f32;

mod logger;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sahister".to_string(),
            width: 1024.,
            height: 1024.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup)
        )
        .add_system_set(
            SystemSet::new()
                .with_system(create_chessboard)
                .with_system(execute_destroy_chessboard)
                .with_system(camera_mover)
        )
        .run();
}

#[derive(Component)]
struct ChessTile {
    row: u8,
    col: u8,
}

#[derive(Default)]
struct MapTiles {
    black_tile: Handle<Image>,
    white_tile: Handle<Image>,
}

#[derive(Default)]
struct PrevCursorPos {
    pos: Option<Vec2>,
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.33;

    commands
        .spawn_bundle(camera)
        .insert(MainCamera);

    commands.insert_resource(PrevCursorPos {
        pos: None,
    });

    commands.insert_resource(MapTiles {
        black_tile: asset_server.load(&(NORMAL_MAP_TILE_PATH.to_string() + "map_tile_dark.png")),
        white_tile: asset_server.load(&(NORMAL_MAP_TILE_PATH.to_string() + "map_tile_white.png")),
    });
}

fn create_chessboard(
    mut commands: Commands,
    current_map_tiles: Res<MapTiles>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::B) {
        logger::log("Creating chessboard");
        let compute_offset: f32 = -(GRID as i8 / 2) as f32 * TILE_DIM as f32;
        for row in 0..GRID {
            for col in 0..GRID {
                let map_tile = if (col + row) % 2 == 0 {current_map_tiles.black_tile.clone()} else {current_map_tiles.white_tile.clone()};
                commands.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(col as f32 * TILE_DIM as f32 + compute_offset, row as f32 * TILE_DIM as f32 + compute_offset, 0.0),
                        ..Default::default()
                    },
                    texture: map_tile,
                    ..Default::default()
                }).insert(ChessTile {
                    row,
                    col,
                });
            }
        }
    }
}

fn destroy_chessboard(
    mut commands: Commands,
    tiles_query: Query<Entity, With<ChessTile>>,
) {
    logger::log("Destroying chessboard");
    for tile_entity in tiles_query.iter() {
        commands.entity(tile_entity).despawn();
    }
}

fn execute_destroy_chessboard(
    input: Res<Input<KeyCode>>,
    commands: Commands,
    tiles_query: Query<Entity, With<ChessTile>>,
) {
    if input.just_pressed(KeyCode::A) {
        destroy_chessboard(commands, tiles_query)
    }
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