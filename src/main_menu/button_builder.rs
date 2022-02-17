use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.4, 0.4);
pub const PRESSED_BUTTON: Color = Color::rgb(0.6, 0.2, 0.3);

pub struct ButtonBuilder;

impl ButtonBuilder {
    pub fn build_button<T: Component + ClassicButton>(
        commands: &mut ChildBuilder, 
        button: T,
        asset_server: &Res<AssetServer>,
    ) {
        commands.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(75.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: UiImage(asset_server.load("./sprites/ui/button.png")),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    button.get_button_type().get_type_name(),
                    TextStyle {
                        font: asset_server.load("fonts/Symtext.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default()
                ),
                ..Default::default()
            });
        }).insert(button);
    }
}

pub trait ButtonType {
    fn get_type_name(&self) -> String;
}

pub trait ClassicButton {
    fn get_button_name(&self) -> String;

    fn get_button_type(&self) -> Box<dyn ButtonType>;
}