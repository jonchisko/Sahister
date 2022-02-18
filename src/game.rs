use std::collections::HashMap;

use bevy::prelude::*;
use crate::SkinSetResource;
use crate::app_states::AppState;
use crate::camera_controller::CameraControllerPlugin;
use crate::logger;

const GRID: u8 = 8;
const TILE_DIM: u8 = 32;

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
struct CurrentSkinSet {
    chessboard: HashMap<&'static str, Handle<Image>>,
    figures: HashMap<&'static str, Handle<Image>>,
}

#[derive(Component)]
struct ChessTile {
    col: u8,
    row: u8,
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

    commands.insert_resource(current_skins);
}

fn update_ingame(

) {
    // TODO @jonchisko 
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
    current_skins.chessboard.insert("BLACK", asset_server.load(&(tiles_path.clone() + "map_tile_dark.png")));
    current_skins.chessboard.insert("WHITE", asset_server.load(&(tiles_path.clone() + "map_tile_white.png")));

    let figures_path = skin_set.figures[skin_set.selected_figures.as_ref().expect("No selected figures")].clone();
    current_skins.figures.insert("BLACK_PAWN", asset_server.load(&(figures_path.clone() + "black/pawn.png")));
    current_skins.figures.insert("BLACK_KING", asset_server.load(&(figures_path.clone() + "black/king.png")));
    current_skins.figures.insert("BLACK_QUEEN", asset_server.load(&(figures_path.clone() + "black/queen.png")));
    current_skins.figures.insert("BLACK_KNIGHT", asset_server.load(&(figures_path.clone() + "black/knight.png")));
    current_skins.figures.insert("BLACK_FORT", asset_server.load(&(figures_path.clone() + "black/fort.png"))); // rook u nub @jonchisko
    current_skins.figures.insert("BLACK_BISHOP", asset_server.load(&(figures_path.clone() + "black/bishop.png")));
    current_skins.figures.insert("WHITE_PAWN", asset_server.load(&(figures_path.clone() + "white/pawn.png")));
    current_skins.figures.insert("WHITE_KING", asset_server.load(&(figures_path.clone() + "white/king.png")));
    current_skins.figures.insert("WHITE_QUEEN", asset_server.load(&(figures_path.clone() + "white/queen.png")));
    current_skins.figures.insert("WHITE_KNIGHT", asset_server.load(&(figures_path.clone() + "white/knight.png")));
    current_skins.figures.insert("WHITE_FORT", asset_server.load(&(figures_path.clone() + "white/fort.png")));
    current_skins.figures.insert("WHITE_BISHOP", asset_server.load(&(figures_path.clone() + "white/bishop.png")));
    
    current_skins
}

fn construct_chessboard(
    commands: &mut Commands,
    current_skins: &CurrentSkinSet,
) {
    let compute_offset: f32 = -(GRID as i8 / 2) as f32 * TILE_DIM as f32;
    for row in 0..GRID {
        for col in 0..GRID {
            let map_tile = if (col + row) % 2 == 0 {
                current_skins.chessboard["BLACK"].clone()
            } else {
                current_skins.chessboard["WHITE"].clone()
            };
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