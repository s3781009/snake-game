use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use food::FoodPlugin;
use game::{GamePlugin, GameState};
use menu::MenuPlugin;
use snake::SnakePlugin;
mod food;
mod game;
mod menu;
mod snake;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        .add_plugin(AudioPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(FoodPlugin)
        .run();
}
