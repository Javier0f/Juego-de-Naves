use hecs::World;
use macroquad::{
    input::{is_key_down, KeyCode},
    math::*, 
    window::{screen_width, screen_height},
    color::*,
    shapes::*,
    rand
};

struct Asteroid;
struct Position(Vec2);
struct Size(f32);
struct Sides(u8);
struct Speed(f32);
struct Direction(Vec2);
struct Rotation(f32);

const ASTEROID_COLOR_1: Color = Color::new(1.0, 0.07, 0.43, 1.0);
const ASTEROID_COLOR_2: Color = Color::new(0.98, 1.0, 0.07, 1.0);

const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

pub fn add_asteroid(world: &mut World){
    let rand_size = rand::gen_range(30.0, 90.0);
    let rand_x = rand::gen_range(rand_size, SCREEN_WIDTH - rand_size);
    let rand_y = rand::gen_range(rand_size, SCREEN_HEIGHT - rand_size);
    let rand_sides = rand::gen_range(5, 11);
    let rand_dir_x = rand::gen_range(-1.0, 1.0);
    let rand_dir_y = rand::gen_range(-1.0, 1.0);
    let rand_speed = rand::gen_range(60.0, 90.0);

    world.spawn((
        Asteroid,
        Position(vec2(rand_x, rand_y)),
        Direction(vec2(rand_dir_x, rand_dir_y)),
        Size(rand_size),
        Sides(rand_sides),
        Speed(rand_speed),
        Rotation(0.0),
    ));
}

pub fn asteroid_movement(world: &mut World, dt: f32){
    world.query_mut::<(&Asteroid, &mut Position, &Direction, &Size, &mut Rotation, &Speed, &Sides)>().into_iter().for_each(|(_type, pos, dir, size, rot, speed, sides)|{
        pos.0 += dir.0 * speed.0 * dt;

        rot.0 += speed.0 * dt * dir.0.x;
        rot.0 %= 360.0;

        if pos.0.x < -size.0 {
            pos.0.x = SCREEN_WIDTH + size.0;
        }
        if pos.0.x > (SCREEN_WIDTH + size.0){
            pos.0.x = -size.0;
        }

        if pos.0.y < -size.0 {
            pos.0.y = SCREEN_HEIGHT + size.0;
        }
        if pos.0.y > (SCREEN_HEIGHT + size.0){
            pos.0.y = -size.0;
        }

        draw_poly_lines(pos.0.x, pos.0.y, sides.0, size.0, rot.0, 8.5, ASTEROID_COLOR_1);
        draw_poly_lines(pos.0.x, pos.0.y, sides.0, size.0 - (size.0 / 4.0), rot.0, 8.5, ASTEROID_COLOR_2);
    })
}