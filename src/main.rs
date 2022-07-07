use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use food::FoodPlugin;
use game::GameState;
use menu::MenuPlugin;
use rand::prelude::random;
use snake::SnakePlugin;
mod food;
mod game;
mod menu;
mod snake;

fn main() {
    App::new()
        .add_state(GameState::Menu)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..default()
        })
        .add_startup_system(game::setup_camera)
        .add_system(game::game_over.after(snake::snake_movement))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(game::position_translation)
                .with_system(game::size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(FoodPlugin)
        .add_system(game::spawn_score)
        .add_system(game::update_score)
        .run();
}
