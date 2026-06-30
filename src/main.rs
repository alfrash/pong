use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;

#[derive(Component)]
#[require(Position)]
struct Paddle;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup,
            (spawn_camera, spawn_ball, spawn_paddle)
        )
        .add_systems(
            FixedUpdate,
            (move_ball.before(project_positions), project_positions),
        )
        .run();
}
const BALL_SIZE: f32 = 20.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1.0, 0., 0.);
fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // `Assets::add` will load these into memory and return a `Handle` (an ID)
    // to these assets. When all references to this `Handle` are cleaned up
    // the asset is cleaned up.
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0)));
}

//Take our own Position and update Bevy's generic Transform to keep them in sync.
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    // Here we are iterating over the query to get the
    // components from our game world
    for (mut transform, position) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.0);
    }
}

fn move_ball(mut position: Single<&mut Position, With<Ball>>) {
    position.0.x += 1.0;
}

const PADDLE_SHAPE:Rectangle = Rectangle::new(20., 50.);
const PADDLE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);

    commands.spawn((
        Paddle,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(250.0, 0.0))
    ));
}