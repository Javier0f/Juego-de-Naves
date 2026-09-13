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
const SPAWN_DISTANCE: f32 = 55.0;

pub fn bullets(world: &mut World, dt: f32, cooldown: &mut u8){
    let mut bullet = (
        Bullet,
        Position(Vec2::ZERO),
        Direction(Vec2::ZERO),
        Life(BULLET_LIFE),
        Size(5.0),
    );

    let mut bullets_despawn: Vec<Entity> = vec![];

    if is_key_down(KeyCode::A) && *cooldown == 0{
        world.query::<With<(&Position, &Rotation), &Player>>()
        .iter()
        .for_each(|(pos, rot)|{
            let rad = rot.0.to_radians();

            let new_pos = vec2(
                pos.0.x + rad.sin() * SPAWN_DISTANCE,
                pos.0.y - rad.cos() * SPAWN_DISTANCE
            );

            bullet.1.0 = new_pos;
            bullet.2.0.y = -rad.cos() + rand::gen_range(-0.1,0.1);
            bullet.2.0.x = rad.sin() + rand::gen_range(-0.1,0.1);
            bullet.2.0 = bullet.2.0.normalize();
        });
        world.spawn(bullet);
        *cooldown = MAX_COOLDOWN;
    }

    if *cooldown > 0{
        *cooldown -= 1;
    }


    world.query_mut::<With<(Entity, &mut Position, &mut Direction, &mut Life), &Bullet>>()
    .into_iter()
    .for_each(|(e, pos, dir, life)|{
        if life.0 > 0{
            draw_circle(pos.0.x, pos.0.y, 10.0, RED);

            life.0 -= 1;
            pos.0 += dir.0 * BULLET_SPEED * dt;
        }

        if life.0 == 0 {
            bullets_despawn.push(e);
        }
    });

    if bullets_despawn.len() > 0 {
        bullets_despawn.iter().for_each(|e|{
            let _ =world.despawn(*e);
        });
        bullets_despawn.clear();
    }

    let mut collision = [(Vec2::ZERO, 0.0);30];
    let mut index: usize = 0;

    world.query::<With<(&Position, &Size), &Asteroid>>()
    .iter()
    .for_each(|(pos, size)| {
        collision[index] = (pos.0,size.0);
        index += 1;
    });

    world.query_mut::<With<(&Position, &mut Life),&Bullet>>()
    .into_iter()
    .for_each(|(pos, life)|{
        collision.iter().for_each(|(pos_a, size)|{
            // let dis = pos.0.distance(*pos_a);
            if (pos.0.distance(*pos_a) - size) < 0.0{
                life.0 = 0;
            }
        });
    });

}
