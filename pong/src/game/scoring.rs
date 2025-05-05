use bevy::prelude::*;

use crate::game::{Position, Velocity};
use crate::game::ball::Ball;

enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
pub(crate) struct Scored(Scorer);

#[derive(Resource, Default)]
pub(crate) struct Score {
    player: u32,
    ai: u32,
}

pub(crate) fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.single(){
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.single_mut() {
            if ball.0.x > window_width / 2. {
                events.write(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.write(Scored(Scorer::Player));
            }
        }
    }
}

pub(crate) fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut ball_position, mut ball_velocity)) = ball.single_mut() {
            match event.0 {
                Scorer::Ai => {
                    ball_position.0 = Vec2::new(0., 0.);
                    ball_velocity.0 = Vec2::new(-1., 1.);
                }
                Scorer::Player => {
                    ball_position.0 = Vec2::new(0., 0.);
                    ball_velocity.0 = Vec2::new(1., 1.);
                }
            }
        }
    }
}

pub(crate) fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => {
                score.ai += 1;
            }
            Scorer::Player => {
                score.player += 1;
            }
        }
    }
}