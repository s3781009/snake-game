use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use rand::prelude::random;
mod food;
mod game;
mod snake;

fn show_score(mut commands: Commands) {}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..default()
        })
        .add_startup_system(game::setup_camera)
        .add_startup_system(snake::spawn_snake)
        .insert_resource(snake::SnakeSegments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<snake::GrowthEvent>()
        .add_system(snake::snake_movement_input.before(snake::snake_movement))
        .add_event::<game::GameOverEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake::snake_movement)
                .with_system(snake::snake_eating.after(snake::snake_movement))
                .with_system(snake::snake_growth.after(snake::snake_eating)),
        )
        .add_system(game::game_over.after(snake::snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food::food_spawner),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(game::position_translation)
                .with_system(game::size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_system(game::spawn_score)
        .add_system(game::update_score)
        .run();
}
