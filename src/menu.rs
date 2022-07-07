use bevy::prelude::*;

use crate::game::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },

            color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                text: Text::with_section(
                    "New Game",
                    TextStyle {
                        font: asset_server.load("retro-font.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            });
        });
}
fn handle_start_button(
    mut commands: Commands,
    mut interaction_query: Query<(&Children, &Interaction), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
) {
    for (children, interaction) in interaction_query.iter() {
        let child = children.iter().next().unwrap();
        match interaction {
            Interaction::Clicked => {
                state.set(GameState::Game);
            }
            _ => (),
        }
    }
}