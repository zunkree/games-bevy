use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;

use crate::game::{Position, Shape, Velocity};
use crate::game::gutter::GUTTER_HEIGHT;
use crate::game::ai::Ai;
use crate::game::player::Player;

pub(crate) const PADDLE_WIDTH: f32 = 10.0;
pub(crate) const PADDLE_HEIGHT: f32 = 50.0;
pub(crate) const PADDLE_SPEED: f32 = 5.;

#[derive(Component)]
#[require(Position, Velocity, Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))]
pub(crate) struct Paddle;

pub(crate) fn spawn_paddles(
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

pub(crate) fn move_paddles(
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
