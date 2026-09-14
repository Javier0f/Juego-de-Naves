use hecs::{World, With, Entity};
use macroquad::{
    math::*, 
    color::*,
    shapes::*,
    input::*,
    rand,
};

use crate::components::*;

const BULLET_LIFE: i8 = 120;
const BULLET_SPEED: f32 = 450.0;
const MAX_COOLDOWN: u8 = 20;
// const SPAWN_DISTANCE: f32 = 55.0;

pub fn bullets(world: &mut World, dt: f32, cooldown: &mut u8){

    let mut bullets_despawn: Vec<Entity> = vec![];

    if is_key_down(KeyCode::A) && *cooldown == 0{
        let mut last_position : (Vec2, f32) = (Vec2::ZERO, 0.0);
        world.query::<With<(&Position, &Rotation), &Player>>()
        .iter()
        .for_each(|(pos, rot)|{last_position = (pos.0, rot.0);});

        world.spawn((
            Bullet,
            Position(last_position.0),
            Direction(vec2(
                last_position.1.to_radians().sin() + rand::gen_range(-0.1,0.1),
                -last_position.1.to_radians().cos() + rand::gen_range(-0.1,0.1)
            )),
            Life(BULLET_LIFE),
        ));
        *cooldown = MAX_COOLDOWN;
    }

    *cooldown = cooldown.saturating_sub(1);

    world.query_mut::<With<(Entity, &mut Position, &mut Direction, &mut Life), &Bullet>>()
    .into_iter()
    .for_each(|(e, pos, dir, life)|{
        if life.0 > 0{
            draw_circle(pos.0.x, pos.0.y, 10.0, RED);

            life.0 -= 1;
            pos.0 += dir.0 * BULLET_SPEED * dt;
        }else{
            bullets_despawn.push(e);
        }
    });

    bullets_despawn.iter().for_each(|e|{ let _ = world.despawn(*e); });

    let mut collision: Vec<(Vec2,f32)> = vec![];

    world.query::<With<(&Position, &Size), &Asteroid>>()
    .iter()
    .for_each(|(pos, size)| {
        collision.push((pos.0,size.0));
    });

    world.query_mut::<With<(&Position, &mut Life),&Bullet>>()
    .into_iter()
    .for_each(|(pos, life)|{
        collision.iter().for_each(|(pos_a, size)|{
            let is_hit = (pos.0.distance_squared(*pos_a) < size * size )as i8;
            life.0 *= 1 - is_hit;
        });
    });

}
