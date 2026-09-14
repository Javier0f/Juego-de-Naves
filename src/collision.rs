use hecs::{Entity, With, World};
use macroquad::math::*;

use crate::components::*;

pub fn collision_system(world: &mut World) {
    let mut collision: Vec<(Entity, Vec2, Vec2)> = vec![];

    world
        .query::<With<(Entity, &Position, &Size), &Collide>>()
        .iter()
        .for_each(|(enti_a, pos_a, size_a)| {
            world
                .query::<With<(Entity, &Position, &Size), &Collide>>()
                .iter()
                .for_each(|(_enti_b, pos_b, size_b)| {
                    let distance = pos_a.0.distance(pos_b.0);
                    let sum_radios = size_a.0 + size_b.0;

                    if distance < sum_radios && distance > 0.0 {
                        let repulsion = (pos_a.0 - pos_b.0).normalize();
                        let new_pos = vec2(
                            pos_a.0.x + repulsion.x * ((sum_radios - distance) * 0.5),
                            pos_a.0.y + repulsion.y * ((sum_radios - distance) * 0.5),
                        );

                        collision.push((enti_a, repulsion, new_pos));
                    }
                });
        });

    world
        .query_mut::<With<(Entity, &mut Position, &mut Direction), &Collide>>()
        .into_iter()
        .for_each(|(enti_a, pos_a, dir_a)| {
            collision.iter().for_each(|(enti, rep, pos)| {
                if *enti == enti_a {
                    pos_a.0.x = pos.x;
                    pos_a.0.y = pos.y;
                    dir_a.0 = *rep;
                }
            })
        });

    collision.clear();
}
