mod game;

use bevy::prelude::*;

use crate::game::ball::{move_ball, project_positions, spawn_ball};
use crate::game::collision::handle_collisions;
use crate::game::gutter::spawn_gutters;
use crate::game::paddle::{move_paddles, spawn_paddles};
use crate::game::player::handle_player_input;
use crate::game::scoring::{detect_scoring, reset_ball, update_score, Score, Scored};
use crate::game::spawn_camera;

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
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_camera,
                spawn_paddles,
                spawn_gutters
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                handle_player_input,
                detect_scoring,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                move_paddles.after(handle_player_input),
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
