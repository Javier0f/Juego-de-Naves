use hecs::{Entity, With, World};
use macroquad::{color::*, math::*, rand, shapes::*, text::*};

use crate::components::*;

struct Fragment;

const ASTEROID_COLOR_1: Color = Color::new(1.0, 0.07, 0.43, 1.0);
const ASTEROID_COLOR_2: Color = Color::new(0.98, 1.0, 0.07, 1.0);

const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

const FRAGMENT_SPEED: f32 = 50.0;
const FRAGMENT_SIZE: f32 = 20.0;

pub fn add_asteroid(world: &mut World, count: u8) {
    for _i in 0..count {
        let rand_size = rand::gen_range(30.0, 90.0);
        let rand_x = rand::gen_range(rand_size, SCREEN_WIDTH - rand_size);
        let rand_y = rand::gen_range(rand_size, SCREEN_HEIGHT - rand_size);
        let rand_sides = rand::gen_range(5, 11);
        let rand_dir_x = rand::gen_range(-1.0, 1.0);
        let rand_dir_y = rand::gen_range(-1.0, 1.0);
        let rand_speed = rand::gen_range(60.0, 90.0);

        world.spawn((
            Asteroid,
            Collide,
            Position(vec2(rand_x, rand_y)),
            Direction(vec2(rand_dir_x, rand_dir_y)),
            Size(rand_size),
            Sides(rand_sides),
            Speed(rand_speed),
            Rotation(0.0),
            Life(rand_sides as i8),
        ));
    }
}

pub fn asteroid_movement(world: &mut World, dt: f32) {
    let mut delete_pool: Vec<Entity> = vec![];
    let mut spawn_fragment_pool: Vec<Vec2> = vec![];

    world
        .query_mut::<With<
            (
                Entity,
                &Life,
                &mut Position,
                &Direction,
                &Size,
                &mut Rotation,
                &Speed,
                &Sides,
            ),
            &Asteroid,
        >>()
        .into_iter()
        .for_each(|(e, life, pos, dir, size, rot, speed, sides)| {
            if life.0 <= 0 as i8 {
                delete_pool.push(e);
                spawn_fragment_pool.push(pos.0);
            }

            pos.0 += dir.0 * speed.0 * dt;

            rot.0 += speed.0 * dt * dir.0.x;
            rot.0 %= 360.0;

            if pos.0.x < -size.0 {
                pos.0.x = SCREEN_WIDTH + size.0;
            }
            if pos.0.x > (SCREEN_WIDTH + size.0) {
                pos.0.x = -size.0;
            }

            if pos.0.y < -size.0 {
                pos.0.y = SCREEN_HEIGHT + size.0;
            }
            if pos.0.y > (SCREEN_HEIGHT + size.0) {
                pos.0.y = -size.0;
            }

            draw_text(format!("{:?}", life.0), pos.0.x, pos.0.y, 60.0, RED);
            draw_poly_lines(
                pos.0.x,
                pos.0.y,
                sides.0,
                size.0,
                rot.0,
                8.5,
                ASTEROID_COLOR_1,
            );
            draw_poly_lines(
                pos.0.x,
                pos.0.y,
                sides.0,
                size.0 - (size.0 / 4.0),
                rot.0,
                8.5,
                ASTEROID_COLOR_2,
            );
        });

    world
        .query_mut::<With<(&mut Position, &Direction, &Sides, &mut Rotation), &Fragment>>()
        .into_iter()
        .for_each(|(pos, dir, sides, rot)| {
            pos.0 += dir.0 * FRAGMENT_SPEED * dt;

            rot.0 += FRAGMENT_SPEED * dt * dir.0.x;
            rot.0 %= 360.0;

            draw_poly_lines(
                pos.0.x,
                pos.0.y,
                sides.0,
                FRAGMENT_SIZE,
                rot.0,
                8.5,
                ASTEROID_COLOR_1,
            );
        });

    delete_pool.iter().for_each(|e| {
        let _ = world.despawn(*e);
    });

    spawn_fragment_pool.iter().for_each(|pos| {
        for _i in 0..3 {
            world.spawn((
                Fragment,
                Collide,
                Position(*pos),
                Direction(vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0))),
                Sides(rand::gen_range(3, 5)),
                Rotation(0.0),
                Life(1),
            ));
        }
    });
}
