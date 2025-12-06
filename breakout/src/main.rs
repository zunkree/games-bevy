use std::process::CommandArgs;
use bevy::prelude::*;

// Window
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

// Paddle
const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 600.0;

// Ball
const BALL_RADIUS: f32 = 12.0;
const BALL_SPEED: f32 = 500.0;

// Bricks
const BRICK_WIDTH: f32 = 64.0;
const BRICK_HEIGHT: f32 = 24.0;
const BRICK_ROWS: usize = 5;
const BRICK_COLUMNS: usize = 10;
const BRICK_VERTICAL_SPACING: f32 = 4.0;
const BRICK_HORIZONTAL_SPACING: f32 = 4.0;
const BRICK_TOP_OFFSET: f32 = 80.0;

// Walls
const WALL_THICKNESS: f32 = 16.0;

// Gameplay
const INITIAL_LIVES: u8 = 3;

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct Lives(u8);

#[derive(Resource)]
struct CurrentLevel(usize);

#[derive(Resource)]
struct BallLaunched(bool);

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct ColliderCircle(Circle);

impl ColliderCircle {
    fn radius(&self) -> f32 {
        self.0.radius
    }
}

#[derive(Component, Default)]
struct ColliderRectangle(Rectangle);

impl ColliderRectangle {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::ZERO),
    ColliderRectangle = ColliderRectangle(Rectangle {
        half_size: Vec2::new(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
    }),
)]
struct Paddle;

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::ZERO),
    ColliderCircle = ColliderCircle(Circle { radius: BALL_RADIUS }),
)]
struct Ball;

#[derive(Component)]
#[require(
    Position,
    ColliderRectangle = ColliderRectangle(Rectangle {
        half_size: Vec2::new(BRICK_WIDTH / 2.0, BRICK_HEIGHT / 2.0),
    })
)]
struct Brick;

#[derive(Component)]
#[require(
    Position,
    ColliderRectangle,
)]
struct Wall;

#[derive(Component)]
struct Player;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

fn setup_playfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    commands.insert_resource(Score(0));
    commands.insert_resource(Lives(INITIAL_LIVES));
    commands.insert_resource(CurrentLevel(0));
    commands.insert_resource(BallLaunched(false));

    spawn_walls(&mut commands, &mut meshes, &mut materials);
    spawn_paddle(&mut commands);
    spawn_ball(&mut commands);
    spawn_bricks_for_level(&mut commands, 0);

    next_state.set(GameState::Playing);
}

fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    let wall_material = materials.add(Color::srgb(0.3, 0.3, 0.3));
    let wall_shape = Rectangle::new(WALL_THICKNESS, WINDOW_HEIGHT);
    let wall_mesh = meshes.add(wall_shape);

    // Left Wall
    commands.spawn((
        Wall,
        Mesh2d(wall_mesh.clone()),
        MeshMaterial2d(wall_material.clone()),
        Position(Vec2::new(-half_w + WALL_THICKNESS / 2.0, 0.0)),
        ColliderRectangle(wall_shape)
    ));

    // Right Wall
    commands.spawn((
        Wall,
        Mesh2d(wall_mesh.clone()),
        MeshMaterial2d(wall_material.clone()),
        Position(Vec2::new(half_w - WALL_THICKNESS / 2.0, 0.0)),
        ColliderRectangle(wall_shape)
    ));

    // Top Wall
    commands.spawn((
        Wall,
        Mesh2d(wall_mesh.clone()),
        MeshMaterial2d(wall_material.clone()),
        Position(Vec2::new(0.0, half_h - WALL_THICKNESS / 2.0)),
        ColliderRectangle(wall_shape)
    ));
}

fn spawn_paddle(commands: &mut Commands) {}

fn spawn_ball(commands: &mut Commands) {}

fn spawn_bricks_for_level(commands: &mut Commands, level: usize) {}

fn setup_menu() {}

fn menu_input_system() {}

fn pause_toggle_system() {}

fn paddle_input_system() {}

fn movement_system() {}

fn ball_collision_system() {}

fn life_and_level_system() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 120.0))
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnEnter(GameState::Playing), setup_playfield)
        .add_systems(
            Update,
            (
                menu_input_system.run_if(in_state(GameState::Menu)),
                pause_toggle_system,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                paddle_input_system,
                movement_system,
                ball_collision_system,
                life_and_level_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
