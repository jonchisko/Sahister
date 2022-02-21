use bevy::prelude::*;

use super::CurrentSkinSet;


#[derive(Component, Clone, Copy)]
pub struct Figure {
    figure_type: FigureType,
    color: ChessColor,
    current_position: (u8, u8),
}

impl Figure {
    pub fn build_b_pawn(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Pawn, ChessColor::Black, pos)
    }

    pub fn build_b_fort(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Fort, ChessColor::Black, pos)
    }

    pub fn build_b_knight(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Knight, ChessColor::Black, pos)
    }

    pub fn build_b_bishop(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Bishop, ChessColor::Black, pos)
    }

    pub fn build_b_king(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::King, ChessColor::Black, pos)
    }
    
    pub fn build_b_queen(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Queen, ChessColor::Black, pos)
    }

    pub fn build_w_pawn(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Pawn, ChessColor::White, pos)
    }

    pub fn build_w_fort(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Fort, ChessColor::White, pos)
    }

    pub fn build_w_knight(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Knight, ChessColor::White, pos)
    }

    pub fn build_w_bishop(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Bishop, ChessColor::White, pos)
    }

    pub fn build_w_king(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::King, ChessColor::White, pos)
    }
    
    pub fn build_w_queen(pos: (u8, u8)) -> Figure {
        Figure::build(FigureType::Queen, ChessColor::White, pos)
    }

    pub fn get_name(&self) -> String {
        format!("{}_{}", self.color.get_str(), self.figure_type.get_str())
    }

    fn build(figure_type: FigureType, color: ChessColor, current_position: (u8, u8)) -> Figure {
        Figure {
            figure_type,
            color,
            current_position
        }
    }
}

#[derive(Clone, Copy)]
enum FigureType {
    Pawn,
    Fort,
    Bishop,
    Knight,
    King,
    Queen
}

impl FigureType {
    fn get_str(&self) -> &str {
        match self {
            &Self::Pawn => {"PAWN"},
            &Self::Fort => {"FORT"},
            &Self::Bishop => {"BISHOP"},
            &Self::Knight => {"KNIGHT"},
            &Self::King => {"KING"},
            &Self::Queen => {"QUEEN"},
        }
    }
}

#[derive(Clone, Copy)]
enum ChessColor {
    White,
    Black,
}

impl ChessColor {
    fn get_str(&self) -> &str {
        match self {
            &Self::White => {"WHITE"},
            &Self::Black => {"BLACK"},
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct ChessTile {
    color: ChessColor,
    col: u8,
    row: u8,
}

impl ChessTile {
    pub fn build_white(col: u8, row: u8) -> ChessTile {
        ChessTile::build(ChessColor::White, col, row)
    }

    pub fn build_black(col: u8, row: u8) -> ChessTile {
        ChessTile::build(ChessColor::Black, col, row)
    }

    fn build(color: ChessColor, col: u8, row: u8) -> ChessTile {
        ChessTile {
            color,
            col,
            row
        }
    }
}

#[derive(Component)]
pub struct FigureMetadata {
    number_of_moves: u16,
    is_enpassant_valid: bool,
}

pub fn get_figures(
    commands: &mut Commands,
    current_skins: &CurrentSkinSet,
) {
    //BLACK PAWNS
    for col in 0..crate::game::GRID {
        let row = 6u8;
        let b_pawn = Figure::build_b_pawn((col, row));
        commands.spawn_bundle(SpriteBundle{
            transform: Transform {
                translation: crate::game::transform_grid_to_world(col, row, crate::game::TILE_DIM, 0.0),
                ..Default::default()
            },
            texture: current_skins.figures[&b_pawn.get_name()].clone(),
            ..Default::default()
        })
        .insert(b_pawn)
        .insert(FigureMetadata {
            number_of_moves: 0,
            is_enpassant_valid: false,
        });
    }

    let b_fort = Figure::build_b_fort((0, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(0, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_fort.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_fort)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_fort = Figure::build_b_fort((7, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(7, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_fort.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_fort)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_knight = Figure::build_b_knight((1, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(1, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_knight.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_knight)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_knight = Figure::build_b_knight((6, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(6, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_knight.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_knight)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_bishop = Figure::build_b_bishop((2, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(2, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_bishop.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_bishop)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_bishop = Figure::build_b_bishop((5, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(5, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_bishop.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_bishop)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_queen = Figure::build_b_queen((3, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(3, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_queen.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_queen)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let b_king = Figure::build_b_king((4, 7));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(4, 7, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&b_king.get_name()].clone(),
        ..Default::default()
    })
    .insert(b_king)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    //WHITE PAWNS
    for col in 0..crate::game::GRID {
        let row = 1u8;
        let w_pawn = Figure::build_w_pawn((col, row));
        commands.spawn_bundle(SpriteBundle{
            transform: Transform {
                translation: crate::game::transform_grid_to_world(col, row, crate::game::TILE_DIM, 0.0),
                ..Default::default()
            },
            texture: current_skins.figures[&w_pawn.get_name()].clone(),
            ..Default::default()
        })
        .insert(w_pawn)
        .insert(FigureMetadata {
            number_of_moves: 0,
            is_enpassant_valid: false,
        });
    }

    let w_fort = Figure::build_w_fort((0, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(0, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_fort.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_fort)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_fort = Figure::build_w_fort((7, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(7, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_fort.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_fort)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_knight = Figure::build_w_knight((1, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(1, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_knight.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_knight)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_knight = Figure::build_w_knight((6, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(6, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_knight.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_knight)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_bishop = Figure::build_w_bishop((2, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(2, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_bishop.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_bishop)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_bishop = Figure::build_w_bishop((5, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(5, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_bishop.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_bishop)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_queen = Figure::build_w_queen((3, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(3, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_queen.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_queen)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });

    let w_king = Figure::build_w_king((4, 0));
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: crate::game::transform_grid_to_world(4, 0, crate::game::TILE_DIM, 0.0),
            ..Default::default()
        },
        texture: current_skins.figures[&w_king.get_name()].clone(),
        ..Default::default()
    })
    .insert(w_king)
    .insert(FigureMetadata {
        number_of_moves: 0,
        is_enpassant_valid: false,
    });
}