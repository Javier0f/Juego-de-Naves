use hecs::{With, World};
use macroquad::{color::*, math::*, shapes::*};

use crate::components::*;

const DISTANCE: f32 = 40.0;

pub fn lifebar(world: &mut World) {
    let mut pos: Vec2 = Vec2::ZERO;
    let mut rot: f32 = 0.0;

    let mut asteroid_pos = [(vec2(0.0, 0.0), 0.0); 30];
    let mut index: usize = 0;

    world
        .query::<With<(&Position, &Size), &Asteroid>>()
        .iter()
        .for_each(|(pos, size)| {
            asteroid_pos[index] = (pos.0, size.0);
            index += 1;
        });

    world
        .query_mut::<With<(&Position, &Rotation, &mut Life, &Size), &Player>>()
        .into_iter()
        .for_each(|(pos_p, rot_p, life_p, size)| {
            life_p.0 -= asteroid_pos
                .iter()
                .any(|(p, z)| pos_p.0.distance_squared(*p) < (size.0 + z).powi(2) + 100.0)
                as i8;

            pos = pos_p.0;
            rot = rot_p.0.to_radians();
            println!("{:?}", life_p.0);
        });

    let pos1 = vec2(
        pos.x + rot.cos() * -DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * -DISTANCE + rot.cos() * DISTANCE,
    );

    let pos2 = vec2(
        pos.x + rot.cos() * DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * DISTANCE + rot.cos() * DISTANCE,
    );

    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 10.0, BLUE);
}
