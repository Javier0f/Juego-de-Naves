use hecs::{With, World};
use macroquad::{color::*, math::*, shapes::*};

use crate::components::*;

use crate::states::game_state::Game;

const DISTANCE: f32 = 40.0;

const COLOR: Color = Color::new(0.2, 0.3, 1.0, 1.0);

pub fn lifebar(world: &mut World, game: &mut Game) {
    let mut pos: Vec2 = Vec2::ZERO;
    let mut rot: f32 = 0.0;
    let mut life: f32 = 0.0;

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
            if life_p.0 < 0 {
                game.switch(4)
            }

            life_p.0 -= asteroid_pos
                .iter()
                .any(|(p, z)| pos_p.0.distance_squared(*p) < (size.0 + z).powi(2) + 100.0)
                as i8
                * 8;

            pos = pos_p.0;
            rot = rot_p.0.to_radians();
            life = life_p.0 as f32 / MAX_LIFE as f32;
        });

    let pos1 = vec2(
        pos.x + rot.cos() * -DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * -DISTANCE + rot.cos() * DISTANCE,
    );

    let pos2 = vec2(
        pos.x + rot.cos() * DISTANCE - rot.sin() * DISTANCE,
        pos.y + rot.sin() * DISTANCE + rot.cos() * DISTANCE,
    );

    let pos3: Vec2 = vec2(pos.x - rot.sin() * DISTANCE, pos.y + rot.cos() * DISTANCE);

    let pos4: Vec2 = (1.0 - life) * pos3 + life * pos1;

    let pos5: Vec2 = (1.0 - life) * pos3 + life * pos2;

    draw_line(pos3.x, pos3.y, pos4.x, pos4.y, 10.0, COLOR);
    draw_line(pos3.x, pos3.y, pos5.x, pos5.y, 10.0, COLOR);
}
