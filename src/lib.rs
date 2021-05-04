#[allow(dead_code)]
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision.system());
    }
}

pub struct Collision(pub f32);
pub struct PlayerCollision(pub f32);

/// AABB collision type test
///
/// Insert the Player and Collision structs into your components.
fn collision(
    mut player_query: Query<(&PlayerCollision, &mut Transform), With<PlayerCollision>>,
    mut collision_query: Query<(&Collision, &Transform), Without<PlayerCollision>>,
) {
    let (player, mut a_trans) = player_query.single_mut().unwrap();
    let a_rad = Vec3::new(player.0, player.0, player.0);
    let (a_min, a_max) = (a_trans.translation - a_rad, a_trans.translation + a_rad);

    for (col, b_trans) in collision_query.iter_mut() {
        let b_rad = Vec3::new(col.0, col.0, col.0);
        let (b_min, b_max) = (b_trans.translation - b_rad, b_trans.translation + b_rad);

        if a_max.x > b_min.x && a_min.x < b_max.x && a_max.z > b_min.z && a_min.z < b_max.z {
            let (top, bot, left, right) = (
                a_max.z - b_min.z,
                b_max.z - a_min.z,
                a_max.x - b_min.x,
                b_max.x - a_min.x,
            );

            if top < bot && top < left && top < right {
                a_trans.translation.z = a_trans.translation.z.clamp(f32::MIN, b_min.z - a_rad.z);
            }
            if bot < top && bot < left && bot < right {
                a_trans.translation.z = a_trans.translation.z.clamp(b_max.z + a_rad.z, f32::MAX);
            }
            if left < bot && left < top && left < right {
                a_trans.translation.x = a_trans.translation.x.clamp(f32::MIN, b_min.x - a_rad.x);
            }
            if right < bot && right < top && right < top {
                a_trans.translation.x = a_trans.translation.x.clamp(b_max.x + a_rad.x, f32::MAX);
            }
        }
    }
}
