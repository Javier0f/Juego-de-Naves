use hecs::{Entity, With, World};
use macroquad::{color::*, input::*, math::*, rand, shapes::*};

use crate::components::*;

const DISTANCE: f32 = 40.0;

pub fn lifebar(world: &mut World, dt: f32) {
    let mut pos: Vec2 = Vec2::ZERO;
    let mut rot: f32 = 0.0;
    let mut life: i8 = 0;

    world
        .query::<With<(&Position, &Rotation, &Life), &Player>>()
        .iter()
        .for_each(|(pos_p, rot_p, life_p)| {
            pos = pos_p.0;
            rot = rot_p.0.to_radians();
            life = life_p.0;
        });

    let pos1 = vec2(
        pos.x + rot.cos() * -DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * -DISTANCE + rot.cos() * DISTANCE,
    );

    let pos2 = vec2(
        pos.x + rot.cos() * DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * DISTANCE + rot.cos() * DISTANCE,
    );

    println!("{:?}", life);
    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 10.0, BLUE);
}
