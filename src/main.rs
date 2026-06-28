
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball))
        .add_systems(FixedUpdate, project_positions)
        .run();
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Ball);
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn project_positions(
    mut positionables: Query<(&mut Transform, &Position)>,
){
    // Here we are iterating over the query to get the
    // components from our game world
    for (mut transform, positon) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = positon.0.extend(0.0);
    }
}