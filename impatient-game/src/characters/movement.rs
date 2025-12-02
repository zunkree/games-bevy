use crate::characters::animation::*;
use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut AnimationController,
            &mut AnimationState,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    let Ok((mut transform, mut animated, mut state, character)) = query.single_mut() else {
        return;
    };

    let direction = read_movement_input(&input);

    if input.just_pressed(KeyCode::Space) {
        state.is_jumping = true;
        animated.current_animation = AnimationType::Jump;
    }

    let is_running = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if direction != Vec2::ZERO {
        let move_speed = calculate_movement_speed(character, is_running);
        let delta = direction.normalize() * move_speed * time.delta_secs();
        transform.translation += delta.extend(0.0);

        animated.facing = Facing::from_direction(direction);

        if !state.is_jumping {
            state.is_moving = true;
            animated.current_animation = if is_running {
                AnimationType::Run
            } else {
                AnimationType::Walk
            };
        }
    } else if !state.is_jumping {
        state.is_moving = false;
        animated.current_animation = AnimationType::Walk;
    }
}

pub fn update_jump_state(
    mut query: Query<
        (
            &mut AnimationController,
            &mut AnimationState,
            &AnimationTimer,
            &Sprite,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    for (mut animated, mut state, timer, sprite, config) in query.iter_mut() {
        if !state.is_jumping {
            continue;
        }

        let Some(atlas) = sprite.texture_atlas.as_ref() else {
            continue;
        };

        let Some(clip) = animated.get_clip(config) else {
            continue;
        };

        if clip.is_complete(atlas.index, timer.just_finished()) {
            state.is_jumping = false;
            animated.current_animation = AnimationType::Walk;
        }
    }
}

/// Read directional input and return a direction vector
fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [([KeyCode; 2], Vec2); 4] = [
        ([KeyCode::ArrowLeft, KeyCode::KeyA], Vec2::NEG_X),
        ([KeyCode::ArrowRight, KeyCode::KeyD], Vec2::X),
        ([KeyCode::ArrowUp, KeyCode::KeyW], Vec2::Y),
        ([KeyCode::ArrowDown, KeyCode::KeyS], Vec2::NEG_Y),
    ];

    MOVEMENT_KEYS
        .iter()
        .filter(|(key, _)| input.any_pressed(*key))
        .map(|(_, dir)| *dir)
        .sum()
}

fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    }
}
