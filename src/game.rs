use bevy::prelude::*;
pub const ARENA_HEIGHT: u32 = 10;
pub const ARENA_WIDTH: u32 = 10;
pub struct GameOverEvent;
use super::food;
use super::snake;
#[derive(Component)]
pub struct Score;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Menu,
    Game,
}

pub fn spawn_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut segments: ResMut<snake::SnakeSegments>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },

            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                segments.len().to_string(),
                TextStyle {
                    font: asset_server.load("retro-font.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(Score);
}

pub fn update_score(mut commands: Commands, query: Query<Entity, With<Score>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<snake::SnakeSegments>,
    food: Query<Entity, With<food::Food>>,
    segments: Query<Entity, With<snake::SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        snake::spawn_snake(commands, segments_res);
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&snake::Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&snake::Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

// fn init_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     audio.play_looped(asset_server.load("background.mp3"));
// }
