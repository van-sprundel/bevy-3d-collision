// http://graphics.cs.cmu.edu/courses/16-465-s15/code-2.html
#![allow(dead_code)]

use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(aabb_collision.system())
            .add_system(gravity.system())
            .add_system(circle_collision.system());
    }
}

pub struct FloorCollision(pub f32);

pub struct CircleCollision(pub f32);

pub struct AABBCollision(pub f32);

pub struct PlayerCollision(pub f32);

/// AABB collision with direction
///
/// Generally used for Axis-Aligned Rectangles
fn aabb_collision(
    mut player_query: Query<(&PlayerCollision, &mut Transform), With<PlayerCollision>>,
    mut collision_query: Query<(&AABBCollision, &Transform), Without<PlayerCollision>>,
) {
    let (player, mut a_trans) = player_query.single_mut().unwrap();
    let (a_min, a_max) = get_min_max(a_trans.translation,player.0);

    for (col, b_trans) in collision_query.iter_mut() {
        let (b_min, b_max) = get_min_max(b_trans.translation,col.0);

        if is_aabb_colliding(a_max, a_min, b_max, b_min) {
            let (front, back, left, right) = (
                a_max.z - b_min.z,
                b_max.z - a_min.z,
                a_max.x - b_min.x,
                b_max.x - a_min.x,
            );

            if front < back && front < left && front < right {
                a_trans.translation.z = a_trans.translation.z.clamp(f32::MIN, b_min.z - player.0);
            }
            if back < front && back < left && back < right {
                a_trans.translation.z = a_trans.translation.z.clamp(b_max.z + player.0, f32::MAX);
            }
            if left < back && left < front && left < right {
                a_trans.translation.x = a_trans.translation.x.clamp(f32::MIN, b_min.x - player.0);
            }
            if right < back && right < front && right < left {
                a_trans.translation.x = a_trans.translation.x.clamp(b_max.x + player.0, f32::MAX);
            }
        }
    }
}

/// Circular collision
///
/// Generally used for spheres
fn circle_collision(
    mut player_query: Query<(&PlayerCollision, &mut Transform), With<PlayerCollision>>,
    mut collision_query: Query<(&CircleCollision, &Transform), Without<PlayerCollision>>,
) {
    let (player, mut a_trans) = player_query.single_mut().unwrap();
    let (a_min, a_max) = get_min_max(a_trans.translation,player.0);

    for (col, b_trans) in collision_query.iter_mut() {
        let (b_min, b_max) = get_min_max(b_trans.translation,col.0);

        if a_trans.translation.distance(b_trans.translation) < player.0 + col.0 {
            let (front, back, left, right) = (
                a_max.z - b_min.z,
                b_max.z - a_min.z,
                a_max.x - b_min.x,
                b_max.x - a_min.x,
            );

            //TODO find out how to get normal angle value so object can be clamped to x and y.
            if front < back {
                a_trans.translation.z =
                    a_trans.translation.z.clamp(f32::MIN, a_trans.translation.z);
            } else {
                a_trans.translation.z =
                    a_trans.translation.z.clamp(a_trans.translation.z, f32::MAX);
            }
        }
    }
}

/// Gravity collision
///
/// Requires PlayerCollision
fn gravity(
    mut player_query: Query<(&PlayerCollision, &mut Transform), With<PlayerCollision>>,
    mut collision_query: Query<(&FloorCollision, &Transform), Without<PlayerCollision>>,
) {
    let (player, mut a_trans) = player_query.single_mut().unwrap();
    let (a_min, a_max) = get_min_max(a_trans.translation,player.0);

    for (col, b_trans) in collision_query.iter_mut() {
        let (b_min, b_max) = get_min_max(b_trans.translation,col.0);

        a_trans.translation.y -= 0.098;
        if is_aabb_colliding(a_max, a_min, b_max, b_min)
            && a_trans.translation.y > b_trans.translation.y
        {
            a_trans.translation.y = b_trans.translation.y + player.0 * 2.;
        }
    }
}

fn is_aabb_colliding(a_max: Vec3, a_min: Vec3, b_max: Vec3, b_min: Vec3) -> bool {
    a_max.x > b_min.x && a_min.x < b_max.x && a_max.z > b_min.z && a_min.z < b_max.z
}

fn get_min_max(trans: Vec3, range: f32) -> (Vec3, Vec3) {
    let range = Vec3::splat(range);
    (trans - range, trans + range)
}