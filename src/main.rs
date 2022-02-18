use bevy::prelude::*;
use game::GamePlugin;
use std::collections::HashMap;
use main_menu::MainMenuPlugin;


mod logger;
mod camera_controller;
mod app_states;
mod main_menu;
mod game;


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
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup)
        )
        .run();
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
        res.chessboard.insert(ChessboardType::Wooden, String::from("./sprites/map/wood_set/"));

        res.figures.insert(FiguresType::Normal, String::from("./sprites/figures/normal_set/"));
        res.figures.insert(FiguresType::Bw, String::from("./sprites/figures/blackwhite_set/"));
        res.figures.insert(FiguresType::Rb, String::from("./sprites/figures/redblue_set/"));

        res.selected_chessboard = Some(ChessboardType::Normal);
        res.selected_figures = Some(FiguresType::Normal);

        res
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(SkinSetResource::new());
}