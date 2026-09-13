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
const BULLET_SPEED: f32 = 10.0;

struct Bullet;

pub fn bullets(world: &mut World){
    let mut bullet = (
        Bullet,
        Position(Vec2::ZERO),
        Direction(Vec2::ZERO),
        Life(BULLET_LIFE),
    );

    let mut bullets_despawn: Vec<Entity> = vec![];

    let mut cooldown: u8 = 0;

    if is_key_down(KeyCode::A){
        world.query::<With<(&Position, &Rotation), &Player>>()
        .iter()
        .for_each(|(pos, rot)|{
            bullet.1 = *pos;
            bullet.2.0.y = -rot.0.to_radians().cos() + rand::gen_range(-0.1,0.1);
            bullet.2.0.x = rot.0.to_radians().sin() + rand::gen_range(-0.1,0.1);
            bullet.2.0 = bullet.2.0.normalize();
        });
        world.spawn(bullet);
    }


    world.query_mut::<With<(Entity, &mut Position, &Direction, &mut Life), &Bullet>>()
    .into_iter()
    .for_each(|(e, pos, dir, life)|{
        if life.0 > 0{
            draw_circle(pos.0.x, pos.0.y, 10.0, RED);

            life.0 -= 1;
            pos.0 += dir.0 * BULLET_SPEED;
        }

        if life.0 == 0 {
            bullets_despawn.push(e);
        }
        cooldown += 1;
        println!("{}", cooldown);
    });

    if bullets_despawn.len() > 0 {
        bullets_despawn.iter().for_each(|e|{
            let _ =world.despawn(*e);
        });
        bullets_despawn.clear();
    }
}
