use bevy::prelude::*;
use super::button_builder::ClassicButton;

pub struct PanelBuilder;

impl PanelBuilder {
    pub fn build_vertical_panel<T: Component + ClassicButton>(
        parent: &mut ChildBuilder,
        width: f32,
        title: &str,
        asset_server: &Res<AssetServer>,
        buttons: Vec<T>,
        button_creator: fn(&mut ChildBuilder, T, &Res<AssetServer>) -> (),
    ) {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(asset_server.load("./sprites/ui/background.png")),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(95.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        title,
                        TextStyle { font: asset_server.load("./fonts/Symtext.ttf"), font_size: 30.0, color: Color::rgb(0.9, 0.9, 0.9) },
                        Default::default()
                    ),
                    ..Default::default()
                });

                for button in buttons {
                    button_creator(parent, button, &asset_server);
                }
            });
        });
    }
}