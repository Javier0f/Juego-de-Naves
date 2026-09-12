use hecs::{World, With};
use macroquad::{
    math::*, 
    color::*,
    shapes::*,
    rand
};

use crate::components::*;

const BULLET_LIFE: f32 = 20.0;
const BULLET_SPEED: f32 = 30.0;

struct Bullet;
struct Index(usize);

pub struct Bullets{
    index: usize,
}

impl Bullets{
    pub fn new(world: &mut World)->Bullets{

        for i in 0..30{
            world.spawn((
                Bullet,
                Index(i),
                Position(Vec2::ZERO),
                Direction(Vec2::ZERO),
                Speed(BULLET_SPEED),
                Life(0),
            ));
        }
        Bullets{
            index: 0
        }
    }

    pub fn process(&mut self, world: &mut World){
        world.query::<With<(&Position, &Rotation), &Player>>()
        .iter()
        .for_each(|(pos, rot)|{
            println!("{}  {}", pos.0, rot.0);
        });
    }
}