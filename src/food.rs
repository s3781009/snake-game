use crate::game::GameState;

use super::{game, snake};
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            // work around for timestep not respecting the current state, only spawn food when in game state
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0).chain(
                    |In(input): In<ShouldRun>, state: Res<State<GameState>>| {
                        if state.current() == &GameState::Game {
                            input
                        } else {
                            ShouldRun::No
                        }
                    },
                ))
                .with_system(food_spawner),
        );
    }
}
#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(snake::Position {
            x: (random::<f32>() * game::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * game::ARENA_HEIGHT as f32) as i32,
        })
        .insert(snake::Size::square(0.8));
}
