use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct CollisionPlugin;


impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision.system());
    }
}

pub struct Collision;

pub struct Size(pub f32);

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

fn collision(
    mut player_query: Query<(&mut Player, &Transform, &Size), With<Player>>,
    entity_query: Query<(&Transform, &Size), With<Collision>>,
) {
    if let Ok((mut player, player_transform, size)) = player_query.single_mut() {
        player.right_speed = 1.0;
        player.left_speed = 1.0;
        player.forward_speed = 1.0;
        player.backward_speed = 1.0;
        entity_query.for_each(|(entity_transform, size2)| {
            let p_radius = Vec3::new(size.0 / 2., size.0 / 2., size.0 / 2.);
            let e_radius = Vec3::new(size2.0 / 2., size2.0 / 2., size2.0 / 2.);
            let (p_min, p_max) = (player_transform.translation - p_radius, player_transform.translation + p_radius);
            let (e_min, e_max) = (entity_transform.translation - e_radius, entity_transform.translation + e_radius);

            // todo check facing side to determine blocking direction.
            if p_max.x > e_min.x && p_min.x < e_max.x && p_max.z > e_min.z && p_min.z < e_max.z {
                match (
                    p_max.x - e_min.x < e_max.x - p_min.x, // left side
                    p_max.z - e_min.z < e_max.z - p_min.z, // forward side
                ) {
                    (true, true) => {
                        player.right_speed = 0.0;
                        player.backward_speed = 0.0;

                        println!("-x -z")
                    }
                    (false, true) => {
                        player.left_speed = 0.0;
                        player.backward_speed = 0.0;
                        println!("+x -z")
                    }
                    (true, false) => {
                        player.right_speed = 0.0;
                        player.forward_speed = 0.0;
                        println!("-x +z")
                    }
                    (false, false) => {
                        player.left_speed = 0.0;
                        player.forward_speed = 0.0;
                        println!("+x +z")
                    }
                }
            }
        });
    }
}