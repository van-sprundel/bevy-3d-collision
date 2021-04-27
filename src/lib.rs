#[allow(dead_code)]
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(collision.system());
    }
}

pub struct Collision;
pub struct Rad(pub f32);

pub struct Player {
    pub left_speed: f32,
    pub right_speed: f32,
    pub forward_speed: f32,
    pub backward_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            left_speed: 1.,
            right_speed: 1.,
            forward_speed: 1.,
            backward_speed: 1.,
        }
    }
}

/// AABB collision type test
///
/// Insert the Player and Collision structs into your components.
fn collision(
    mut player_query: Query<(&mut Player, &Transform, &Rad), With<Player>>,
    entity_query: Query<(&Transform, &Rad), With<Collision>>,
) {
    if let Ok((mut player, a_trans, rad)) = player_query.single_mut() {
        player.right_speed = 1.0;
        player.left_speed = 1.0;
        player.forward_speed = 1.0;
        player.backward_speed = 1.0;

        let a_rad = Vec3::new(rad.0 / 2., rad.0 / 2., rad.0 / 2.);
        let (a_min, a_max) = (a_trans.translation - a_rad, a_trans.translation + a_rad);

        entity_query.for_each(|(b_trans, rad2)| {
            let b_rad = Vec3::new(rad2.0 / 2., rad2.0 / 2., rad2.0 / 2.);
            let (b_min, b_max) = (b_trans.translation - b_rad, b_trans.translation + b_rad);

            if a_max.x > b_min.x && a_min.x < b_max.x && a_max.z > b_min.z && a_min.z < b_max.z {
                let (top, bot, left, right) = (
                    a_max.z - b_min.z,
                    b_max.z - a_min.z,
                    a_max.x - b_min.x,
                    b_max.x - a_min.x,
                );

                if top < bot && top < left && top < right {
                    player.backward_speed = 0.0
                }
                if bot < top && bot < left && bot < right {
                    player.forward_speed = 0.0
                }
                if left < bot && left < top && left < right {
                    player.right_speed = 0.0
                }
                if right < bot && right < top && right < top {
                    player.left_speed = 0.0
                }
            }
        });
    }
}