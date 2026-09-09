use hecs::{World, With};
use macroquad::{
    input::{is_key_down, KeyCode},
    math::*, 
    window::{screen_width, screen_height},
    color::*,
    shapes::*
};

use crate::components::*;

const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

const PLAYER_COLOR_1: Color = Color::new(1.0 , 0.52 , 0.32 ,1.0);
const PLAYER_COLOR_2: Color = Color::new(0.01 , 0.71 , 0.66 , 1.0);
const MAX_LIFE: i8 = 100;
const PLAYER_SIZE: f32 = 30.0;
const PLAYER_SPEED: f32 = 5.0;

pub fn add_player(world: &mut  World){
    let _ = world.spawn((
        Player,
        Position(vec2((screen_width() + PLAYER_SIZE) / 2.0, (screen_height() + PLAYER_SIZE) / 2.0)),
        Direction(Vec2::ZERO),
        Size(PLAYER_SIZE),
        Life(MAX_LIFE),
        OnMove(false),
        Rotation(0.0),
    ));
}

pub fn player_movement(world: &mut World, dt: f32){
    // world.query_mut::<With<(&mut Position, &mut Direction, &mut Rotation), &Player>>()
    world.query_mut::<With::<(&mut OnMove, &mut Position, &mut Direction, &mut Rotation), &Player>>()
    .into_iter()
    .for_each(|(onmove, pos, dir, rot)|{
        let mut dire = Vec2::ZERO;
        let rot_rad = rot.0.to_radians();
        onmove.0 = false;

        if pos.0.x > SCREEN_WIDTH + PLAYER_SIZE{
            pos.0.x = -PLAYER_SIZE
        }
        if pos.0.x < -PLAYER_SIZE -3.0{
            pos.0.x = SCREEN_WIDTH + PLAYER_SIZE
        }
        if pos.0.y > SCREEN_HEIGHT + PLAYER_SIZE{
            pos.0.y = -PLAYER_SIZE
        }
        if pos.0.y < -PLAYER_SIZE -3.0{
            pos.0.y = SCREEN_HEIGHT + PLAYER_SIZE
        }

        if is_key_down(KeyCode::Up){
            dire.x = rot_rad.sin();
            dire.y = -rot_rad.cos();
            onmove.0 = true;
        }
        if is_key_down(KeyCode::Down){
            dire.x = -rot_rad.sin();
            dire.y = rot_rad.cos();
            onmove.0 = true;
        }
        if is_key_down(KeyCode::Right){rot.0 += 3.0}
        if is_key_down(KeyCode::Left){rot.0 -= 3.0}

        if dire.length_squared() > 0.0 {dire = dire.normalize()}

        let target = dire * PLAYER_SPEED;

        dir.0 = dir.0.lerp(target, 1.0 * dt);

        pos.0.x += dir.0.x;
        pos.0.y += dir.0.y;
    });

    world.query::<(&Player, &Position, &Size, &Rotation)>().iter().for_each(|(_player, pos, size, rot)|{
        let sen_a = rot.0.to_radians().sin();
        let cos_a = rot.0.to_radians().cos();

        let h = size.0 ;
        let b = size.0 * 1.3;

        let vec_a = vec2(
            pos.0.x + h * sen_a,
            pos.0.y - h * cos_a
        );

        let vec_b = vec2(
            pos.0.x - cos_a * b - sen_a * h,
            pos.0.y - sen_a * b + cos_a * h
        );


        let vec_c = vec2(
            pos.0.x + cos_a * b - sen_a * h,
            pos.0.y + sen_a * b + cos_a * h
        );

        draw_poly_lines(pos.0.x, pos.0.y, 3, size.0 + 3.0, rot.0 + 30.0 , 14.0, PLAYER_COLOR_1);
        draw_triangle(vec_a, vec_b, vec_c, PLAYER_COLOR_2);
    })
}