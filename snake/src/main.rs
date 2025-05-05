use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const WINDOW_WIDTH: f32 = 521.0;
const WINDOW_HEIGHT: f32 = 521.0;
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(10.0, 10.0)))
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(side: f32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    match windows.single() {
        Ok(window) => {
            for (size, mut transform) in q.iter_mut() {
                println!("org size: {:?}, window: {:?}", size, window);
                transform.scale = Vec3::new(
                    size.width / ARENA_WIDTH as f32 * window.width(),
                    size.height / ARENA_HEIGHT as f32 * window.height(),
                    1.0,
                );
                println!("res size: {:?}, window: {:?}", size, window);
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    }
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + title_size / 2.0
    }
    match windows.single() {
        Ok(window) => {
            for (position, mut transform) in q.iter_mut() {
                transform.translation = Vec3::new(
                    convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                    convert(
                        position.y as f32,
                        window.height() as f32,
                        ARENA_HEIGHT as f32,
                    ),
                    0.0,
                );
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement)
        .add_systems(PostUpdate, size_scaling)
        .add_systems(PostUpdate, position_translation)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .run();
}
