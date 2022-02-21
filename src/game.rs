use std::collections::HashMap;

use bevy::prelude::*;
use crate::SkinSetResource;
use figures::{ChessTile, Figure};
use crate::app_states::AppState;
use crate::camera_controller::CameraControllerPlugin;
use crate::logger;

const GRID: u8 = 8;
const TILE_DIM: u8 = 32;

mod figures;
mod clicker;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraControllerPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                .with_system(setup_ingame)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(update_ingame)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                .with_system(exit_ingame)
            );
    }
}

#[derive(Default)]
pub struct CurrentSkinSet {
    chessboard: HashMap<String, Handle<Image>>,
    figures: HashMap<String, Handle<Image>>,
}

fn setup_ingame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    skin_set: Res<SkinSetResource>,
) {
    logger::log("Creating chessboard");
    let current_skins = load_current_skins(
        &asset_server,
        &skin_set
    );

    construct_chessboard(
        &mut commands,
        &current_skins
    );

    add_figures(
        &mut commands,
        &current_skins
    );

    commands.insert_resource(current_skins);
}

fn update_ingame(
    test_query: Query<(&Transform, &Figure)>,
) {

}

fn exit_ingame(
    mut commands: Commands,
    tiles_query: Query<Entity, With<ChessTile>>,
) {
    logger::log("Destroying chessboard");
    for tile_entity in tiles_query.iter() {
        commands.entity(tile_entity).despawn();
    }
}

fn load_current_skins(
    asset_server: &Res<AssetServer>,
    skin_set: &Res<SkinSetResource>,
) -> CurrentSkinSet {
    let mut current_skins = CurrentSkinSet {
        chessboard: HashMap::new(),
        figures: HashMap::new(),
    };

    let tiles_path = skin_set.chessboard[skin_set.selected_chessboard.as_ref().expect("No selected chessboard")].clone();
    current_skins.chessboard.insert("BLACK".to_string(), asset_server.load(&(tiles_path.clone() + "map_tile_dark.png")));
    current_skins.chessboard.insert("WHITE".to_string(), asset_server.load(&(tiles_path.clone() + "map_tile_white.png")));

    let figures_path = skin_set.figures[skin_set.selected_figures.as_ref().expect("No selected figures")].clone();
    logger::log(&figures_path);
    current_skins.figures.insert("BLACK_PAWN".to_string(), asset_server.load(&(figures_path.clone() + "black/pawn.png")));
    current_skins.figures.insert("BLACK_KING".to_string(), asset_server.load(&(figures_path.clone() + "black/king.png")));
    current_skins.figures.insert("BLACK_QUEEN".to_string(), asset_server.load(&(figures_path.clone() + "black/queen.png")));
    current_skins.figures.insert("BLACK_KNIGHT".to_string(), asset_server.load(&(figures_path.clone() + "black/knight.png")));
    current_skins.figures.insert("BLACK_FORT".to_string(), asset_server.load(&(figures_path.clone() + "black/fort.png"))); // rook u nub @jonchisko
    current_skins.figures.insert("BLACK_BISHOP".to_string(), asset_server.load(&(figures_path.clone() + "black/bishop.png")));
    current_skins.figures.insert("WHITE_PAWN".to_string(), asset_server.load(&(figures_path.clone() + "white/pawn.png")));
    current_skins.figures.insert("WHITE_KING".to_string(), asset_server.load(&(figures_path.clone() + "white/king.png")));
    current_skins.figures.insert("WHITE_QUEEN".to_string(), asset_server.load(&(figures_path.clone() + "white/queen.png")));
    current_skins.figures.insert("WHITE_KNIGHT".to_string(), asset_server.load(&(figures_path.clone() + "white/knight.png")));
    current_skins.figures.insert("WHITE_FORT".to_string(), asset_server.load(&(figures_path.clone() + "white/fort.png")));
    current_skins.figures.insert("WHITE_BISHOP".to_string(), asset_server.load(&(figures_path.clone() + "white/bishop.png")));
    
    current_skins
}

fn construct_chessboard(
    commands: &mut Commands,
    current_skins: &CurrentSkinSet,
) {
    
    for row in 0..GRID {
        for col in 0..GRID {
            let tile_component;
            let map_tile = if (col + row) % 2 == 0 {
                tile_component = ChessTile::build_black(col, row);
                current_skins.chessboard["BLACK"].clone()
            } else {
                tile_component = ChessTile::build_white(col, row);
                current_skins.chessboard["WHITE"].clone()
            };
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: transform_grid_to_world(col, row, TILE_DIM, -0.001),
                    ..Default::default()
                },
                texture: map_tile,
                ..Default::default()
            }).insert(tile_component);
        }
    }
}

fn add_figures(
    commands: &mut Commands,
    current_skins: &CurrentSkinSet,
) {
    figures::get_figures(commands, current_skins);
}

fn transform_grid_to_world(col: u8, row: u8, tile_dim: u8, z: f32) -> Vec3 {
    let compute_offset: f32 = -(GRID as i8 / 2) as f32 * TILE_DIM as f32;
    Vec3::new(col as f32 * tile_dim as f32 + compute_offset, row as f32 * tile_dim as f32 + compute_offset, z)
}