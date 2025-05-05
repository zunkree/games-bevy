pub(crate) mod ai;
pub(crate) mod ball;
pub(crate) mod collision;
pub(crate) mod gutter;
pub(crate) mod paddle;
pub(crate) mod player;
pub(crate) mod scoring;

use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub(crate) struct Position(Vec2);

#[derive(Component, Default)]
pub(crate) struct Velocity(Vec2);

#[derive(Component, Default)]
pub(crate) struct Shape(Vec2);

pub(crate) fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera ...");
    commands.spawn(Camera2d);
}
