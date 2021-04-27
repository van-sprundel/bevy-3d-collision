use bevy::prelude::*;
use std::cmp::max;
use core::cmp;
use bevy_playground::{Size,CollisionPlugin,Collision,Player};

const WALK_SPEED: f32 = 0.025;
const RUN_SPEED: f32 = 0.05;

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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15. })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // player
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5,
            depth: 1.,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0., 1., 0.),
        ..Default::default()
    }).insert(Player::default()).insert(Size(1.0));

    // 2 objects
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::default())),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(2., 0.5, 0.),
        ..Default::default()
    }).insert(Collision).insert(Size(1.0));
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(2., 0.5, 3.),
        ..Default::default()
    }).insert(Collision).insert(Size(2.0));

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-5.0, 2.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn movement(keyboard_input: Res<Input<KeyCode>>, query: Query<(&mut Transform, &Player), With<Player>>) {
    query.for_each_mut(|(mut transform, player)| {
        let speed = if keyboard_input.pressed(KeyCode::LShift) { RUN_SPEED } else { WALK_SPEED };

        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.z -= speed * player.forward_speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= speed * player.left_speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.z += speed * player.backward_speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += speed * player.right_speed;
        }
    });
}

