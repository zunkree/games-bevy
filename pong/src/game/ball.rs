use bevy::prelude::*;

use crate::game::{Shape, Position, Velocity};

const BALL_SIZE: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;

#[derive(Component)]
#[require(
    Position,
    Velocity(Vec2::new(-1., 1.)),
    Shape(Vec2::new(BALL_SIZE, BALL_SIZE))
)]
pub(crate) struct Ball;

pub(crate) fn spawn_ball(
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

pub(crate) fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.single_mut() {
        position.0 += velocity.0 * BALL_SPEED
    }
}

pub(crate) fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

