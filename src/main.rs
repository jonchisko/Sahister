use std::collections::HashMap;

use bevy::prelude::*;
use main_menu::MainMenuPlugin;
use crate::camera_controller::CameraControllerPlugin;

const GRID: u8 = 8;
const TILE_DIM: u8 = 32;
const NORMAL_MAP_TILE_PATH: &str = "./sprites/map/normal_set/";

mod logger;
mod camera_controller;
mod app_states;
mod main_menu;


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
        .add_plugin(CameraControllerPlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup)
        )
        .add_system_set(
            SystemSet::new()
                .with_system(create_chessboard)
                .with_system(execute_destroy_chessboard)
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


#[derive(std::cmp::Eq, PartialEq, Hash)]
enum ChessboardType {
    Normal,
    Wooden,
}

#[derive(std::cmp::Eq, PartialEq, Hash)]
enum FiguresType {
    Normal,
    Bw,
    Rb,
}

#[derive(Default)]
struct SkinSetResource {
    chessboard: HashMap<ChessboardType, String>,
    figures: HashMap<FiguresType, String>,
    selected_chessboard: Option<ChessboardType>,
    selected_figures: Option<FiguresType>,
}

impl SkinSetResource {
    fn new() -> SkinSetResource {
        let mut res = SkinSetResource {
            chessboard: HashMap::new(),
            figures: HashMap::new(),
            selected_chessboard: None,
            selected_figures: None,
        };

        res.chessboard.insert(ChessboardType::Normal, String::from("./sprites/map/normal_set/"));
        res.chessboard.insert(ChessboardType::Wooden, String::from("./sprites/map/wooden_set/"));

        res.figures.insert(FiguresType::Normal, String::from("./sprites/figures/normal_set/"));
        res.figures.insert(FiguresType::Bw, String::from("./sprites/figures/blackwhite_set/"));
        res.figures.insert(FiguresType::Rb, String::from("./sprites/figures/redblue_set/"));

        res.selected_chessboard = Some(ChessboardType::Normal);
        res.selected_figures = Some(FiguresType::Normal);

        res
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MapTiles {
        black_tile: asset_server.load(&(NORMAL_MAP_TILE_PATH.to_string() + "map_tile_dark.png")),
        white_tile: asset_server.load(&(NORMAL_MAP_TILE_PATH.to_string() + "map_tile_white.png")),
    });

    commands.insert_resource(SkinSetResource::new());
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