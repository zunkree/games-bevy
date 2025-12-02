use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource, Deref, DerefMut)]
struct PrintTimer(Stopwatch);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(PrintTimer(Stopwatch::new()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, move_player)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Text2d::new("@"),
        TextFont {
            font_size: 12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    for (entity, gamepad) in &gamepads {
        let left_stick_x = match gamepad.get(GamepadAxis::LeftStickX) {
            Some(x) => x,
            None => continue,
        };
        let left_stick_y = match gamepad.get(GamepadAxis::LeftStickY) {
            Some(y) => y,
            None => continue,
        };

        if left_stick_x.abs() < 0.008 && left_stick_y.abs() < 0.008 {
            continue;
        }

        let direction = Vec2::new(left_stick_x, left_stick_y);
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        player_transform.translation += delta.extend(0.0);
    }
}
