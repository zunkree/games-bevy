use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;

use crate::game::{Position, Shape};

pub(crate) const GUTTER_HEIGHT: f32 = 20.;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

pub(crate) fn spawn_gutters(
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
