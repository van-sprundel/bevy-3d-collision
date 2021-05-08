use bevy::prelude::*;
use bevy_collision_3d::{
    AABBCollision, CircleCollision, CollisionPlugin, FloorCollision, PlayerCollision,
};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(FloorCollision(5.));
    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 1.,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..Default::default()
        })
        .insert(PlayerCollision(0.5));

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 5., 5.).looking_at(Vec3::ZERO, -Vec3::Z),
        ..Default::default()
    });
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&mut Transform, With<PlayerCollision>>,
) {
    const SPEED: f32 = 0.025;
    query.for_each_mut(|mut transform| {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.z -= transform.local_z().z * SPEED;
            transform.translation.x -= transform.local_z().x * SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.z -= transform.local_x().z * SPEED;
            transform.translation.x -= transform.local_x().x * SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.z += transform.local_z().z * SPEED;
            transform.translation.x += transform.local_z().x * SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.z += transform.local_x().z * SPEED;
            transform.translation.x += transform.local_x().x * SPEED;
        }
    });
}
