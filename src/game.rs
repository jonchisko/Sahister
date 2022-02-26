use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use crate::SkinSetResource;
use figures::{ChessTile, Figure, ChessColor, FigureType};
use crate::app_states::AppState;
use crate::camera_controller::CameraControllerPlugin;
use clicker::{ClickerGamePlugin, TheTwoSelections};
use crate::logger;

const GRID: i32 = 8;
const TILE_DIM: i32 = 32;

mod figures;
mod clicker;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraControllerPlugin)
            .add_plugin(ClickerGamePlugin)
            .add_event::<LegalMoveEvent>()
            .insert_resource(CurrentPlayer {
                color: Some(ChessColor::White),
            })
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                .with_system(setup_ingame)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(update_ingame)
                .with_system(move_legality)
                .with_system(move_figures)
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

#[derive(Default)]
struct CurrentPlayer {
    color: Option<ChessColor>,
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

fn transform_grid_to_world(col: i32, row: i32, tile_dim: i32, z: f32) -> Vec3 {
    let compute_offset: f32 = -(GRID as i8 / 2) as f32 * TILE_DIM as f32;
    Vec3::new(col as f32 * tile_dim as f32 + compute_offset, row as f32 * tile_dim as f32 + compute_offset, z)
}

fn move_legality(
    mut selection: ResMut<TheTwoSelections>,
    mut legalmove_event: EventWriter<LegalMoveEvent>,
    current_player: Res<CurrentPlayer>,
    figures: Query<&Figure>,
) {    
    if selection.selection1.is_some() && selection.selection2.is_some() {
        let mut move_is_legal = false;
        let (selection1, selection2) = (selection.selection1.take(), selection.selection2.take());
        let (tile1, figure1) = selection1.unwrap();
        let (tile2, figure2) = selection2.unwrap();
    
        let mut king_tile = None;
        for fig in figures.iter() {
            if fig.figure_type == FigureType::King && fig.color == current_player.color.unwrap() {
                king_tile = Some(*fig);
            }
        }
    
        let figure1 = figure1.unwrap();
        let legal_moves = get_legalmoves(figure1.color, figure1.figure_type, tile1);
        if legal_moves.contains(&(tile2.col, tile2.row)) {
            move_is_legal = true;
        }
        match figure1.figure_type {
            FigureType::Pawn => {
    
            },
            FigureType::Fort => {
    
            },
            FigureType::Bishop => {
    
            },
            FigureType::King => {
    
            },
            FigureType::Knight => {
    
            },
            FigureType::Queen => {
    
            }
        }
    
        if move_is_legal {
            legalmove_event.send(LegalMoveEvent{
                move_to: (tile2.col, tile2.row),
                figure: figure1,
            });
        }
    }
}

fn is_tile_free(
    selection: &Res<TheTwoSelections>
) -> bool {
    // always check the second one, the first one always has the figure since you are movin it
    return selection.selection2.unwrap().1.is_none();
}

fn get_legalmoves(
    figure_color: ChessColor,
    figure_type: FigureType,
    tile: ChessTile,
) -> HashSet<(i32, i32)> {
    let mut set_moves = HashSet::new();
    match figure_type {
        FigureType::Pawn => {
            let dir: i32 = match figure_color {
                ChessColor::Black => {
                    -1
                },
                ChessColor::White => {
                    1
                }
            };
            set_moves.insert((tile.col, tile.row + dir)); 
            set_moves.insert((tile.col, tile.row + 2 * dir));
            set_moves.insert((tile.col - 1, tile.row + dir));
            set_moves.insert((tile.col + 1, tile.row + dir));
        },
        FigureType::Fort => {
            let start_pos = (tile.col, tile.row);
            for r in 0..GRID {
                if r != start_pos.1 {
                    set_moves.insert((start_pos.0, r));
                }
            }
            for c in 0..GRID {
                if c != start_pos.0 {
                    set_moves.insert((c, start_pos.1));
                }
            }
        },
        FigureType::Bishop => {
            let start_pos = (tile.col, tile.row);
            for offset in 0..GRID {
                set_moves.insert((start_pos.0 + offset, start_pos.1 + offset)); // right diag up
                set_moves.insert((start_pos.0 + offset, start_pos.1 - offset)); // right diag down
                set_moves.insert((start_pos.0 - offset, start_pos.1 + offset)); // left diag up
                set_moves.insert((start_pos.0 - offset, start_pos.1 - offset)); // left diag down
            }
        },
        FigureType::Knight => {
            let start_pos = (tile.col, tile.row);
            set_moves.insert((start_pos.0 + 2, start_pos.1 + 1));
            set_moves.insert((start_pos.0 + 2, start_pos.1 - 1));
            set_moves.insert((start_pos.0 - 2, start_pos.1 + 1));
            set_moves.insert((start_pos.0 - 2, start_pos.1 - 1));

            set_moves.insert((start_pos.0 + 1, start_pos.1 + 2));
            set_moves.insert((start_pos.0 - 1, start_pos.1 + 2));
            set_moves.insert((start_pos.0 + 1, start_pos.1 - 2));
            set_moves.insert((start_pos.0 - 1, start_pos.1 - 2));
        },
        FigureType::King => {
            let start_pos = (tile.col, tile.row);
            for offset_x in -1..=1 {
                for offset_y in -1..=1 {
                    if offset_x != 0 || offset_y != 0 {
                        set_moves.insert((start_pos.0 + offset_x, start_pos.1 + offset_y));
                    }
                }   
            }
        },
        FigureType::Queen => {
            let start_pos = (tile.col, tile.row);
            for r in 0..GRID {
                if r != start_pos.1 {
                    set_moves.insert((start_pos.0, r));
                }
            }
            for c in 0..GRID {
                if c != start_pos.0 {
                    set_moves.insert((c, start_pos.1));
                }
            }
            for offset in 0..GRID {
                set_moves.insert((start_pos.0 + offset, start_pos.1 + offset)); // right diag up
                set_moves.insert((start_pos.0 + offset, start_pos.1 - offset)); // right diag down
                set_moves.insert((start_pos.0 - offset, start_pos.1 + offset)); // left diag up
                set_moves.insert((start_pos.0 - offset, start_pos.1 - offset)); // left diag down
            }
        }
    }
    set_moves
}

fn is_tile_underattack(
    tile: ChessTile,
    figures: Query<&Figure>,
) -> bool {
    true
}



fn check_nolegal_move(

) -> bool {
    true
}

fn check_check(

) {

}

fn check_checkmate(

) {

}

fn move_figures(
    mut legal_move_event: EventReader<LegalMoveEvent>,
    mut figures: Query<(&mut Transform, &mut Figure)>
) {
    for move_event in legal_move_event.iter() {
        for (mut transform, mut fig) in figures.iter_mut() {
            if move_event.figure == *fig {
                transform.translation = transform_grid_to_world(move_event.move_to.0, move_event.move_to.1, TILE_DIM, 0.0);
                fig.col = move_event.move_to.0;
                fig.row = move_event.move_to.1;
            }
        }
    }
}

fn end_turn(

) {

}

struct LegalMoveEvent {
    move_to: (i32, i32),
    figure: Figure,
}