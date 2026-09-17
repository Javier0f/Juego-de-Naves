use hecs::{Entity, With, World};
use macroquad::{color::*, input::*, math::*, rand, shapes::*};

use crate::components::*;

const BULLET_LIFE: i8 = 120;
const BULLET_SPEED: f32 = 450.0;
const MAX_COOLDOWN: u8 = 20;

pub fn bullets(world: &mut World, dt: f32, cooldown: &mut u8) {
    let mut bullets_despawn: Vec<Entity> = vec![];
    let mut collision: Vec<(Vec2, Entity)> = vec![];

    if is_key_down(KeyCode::A) && *cooldown == 0 {
        let mut last_position: (Vec2, f32) = (Vec2::ZERO, 0.0);
        world
            .query::<With<(&Position, &Rotation), &Player>>()
            .iter()
            .for_each(|(pos, rot)| {
                last_position = (pos.0, rot.0);
            });

        world.spawn((
            Bullet,
            Position(last_position.0),
            Direction(vec2(
                last_position.1.to_radians().sin() + rand::gen_range(-0.1, 0.1),
                -last_position.1.to_radians().cos() + rand::gen_range(-0.1, 0.1),
            )),
            Life(BULLET_LIFE),
        ));
        *cooldown = MAX_COOLDOWN;
    }

    *cooldown = cooldown.saturating_sub(1);

    world
        .query_mut::<With<(Entity, &mut Position, &mut Direction, &mut Life), &Bullet>>()
        .into_iter()
        .for_each(|(e, pos, dir, life)| {
            if life.0 > 0 {
                draw_circle(pos.0.x, pos.0.y, 10.0, RED);
                life.0 -= 1;
                pos.0 += dir.0 * BULLET_SPEED * dt;
                collision.push((pos.0, e));
            } else {
                bullets_despawn.push(e);
            }
        });

    world
        .query_mut::<With<(&Position, &Size, &mut Life), &Asteroid>>()
        .into_iter()
        .for_each(|(pos, size, life)| {
            collision.iter().for_each(|(pos_b, e)| {
                // let is_hit = (pos.0.distance_squared(*pos_b) < size.0 * size.0) as i8;
                if pos.0.distance_squared(*pos_b) < size.0 * size.0 {
                    bullets_despawn.push(*e);
                    life.0 -= 1;
                }
            })
        });

    world
        .query_mut::<With<(&Position, &Size, &mut Life), &Fragment>>()
        .into_iter()
        .for_each(|(pos, size, life)| {
            collision.iter().for_each(|(pos_b, e)| {
                // let is_hit = (pos.0.distance_squared(*pos_b) < size.0 * size.0) as i8;
                if pos.0.distance_squared(*pos_b) < size.0 * size.0 {
                    bullets_despawn.push(*e);
                    life.0 -= 1;
                }
            })
        });

    bullets_despawn.iter().for_each(|e| {
        let _ = world.despawn(*e);
    });

    // world.query::<With<(&Position, &Size), &Asteroid>>()
    // .iter()
    // .for_each(|(pos, size)| {
    //     collision.push((pos.0,size.0));
    // });
    //
    // for (pos, life) in world.query_mut::<With<(&Position, &mut Life), &Bullet>>(){
    //     for (pos_a, size) in collision.iter(){
    //         let is_hit = (pos.0.distance_squared(*pos_a) < size * size )as i8;
    //         life.0 *= 1 - is_hit;
    //     }
    // }
}
