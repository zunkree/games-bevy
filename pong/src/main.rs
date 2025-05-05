use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

const BALL_SIZE: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;
const PADDLE_SPEED: f32 = 5.;

const GUTTER_HEIGHT: f32 = 20.;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Component)]
#[require(
    Position,
    Velocity(Vec2::new(-1., 1.)),
    Shape(Vec2::new(BALL_SIZE, BALL_SIZE))
)]
struct Ball;

#[derive(Component)]
#[require(Position, Velocity, Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))]
struct Paddle;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
struct Scored(Scorer);

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
}

fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera ...");
    commands.spawn(Camera2d);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball ...");
    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1., 0., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.single_mut() {
        position.0 += velocity.0 * BALL_SPEED
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles ...");
    if let Ok(window) = window.single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        let mesh = meshes.add(shape);
        let player_color = materials.add(Color::srgb(0., 1., 0.));
        let ai_color = materials.add(Color::srgb(0., 0., 1.));

        commands.spawn((
            Player,
            Paddle,
            Position(Vec2::new(left_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(player_color.clone()),
        ));

        commands.spawn((
            Ai,
            Paddle,
            Position(Vec2::new(right_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ai_color.clone()),
        ));
    }
}

fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center);
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left | Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top | Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
        let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

        let shape = Rectangle::from_size(Vec2::new(window_width, GUTTER_HEIGHT));
        let color = Color::srgb(0., 0., 0.);

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0., top_gutter_y)),
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
        ));

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0., bottom_gutter_y)),
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
        ));
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (spawn_ball, spawn_camera, spawn_paddles, spawn_gutters),
        )
        .add_systems(
            Update,
            (
                move_ball,
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
                handle_player_input,
                move_paddles,
            ),
        )
        .run();
}
