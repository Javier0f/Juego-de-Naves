
use hecs::{World, Entity};
use macroquad::{
    math::*, 
    color::*,
    shapes::*
};

use crate::components::*;

const MAX_PARTICLES: usize = 50;
const PARTICLE_LIFE: f32 = 10.0;
const PARTICLE_SPEED: f32 = 50.0;

#[derive(Debug, Copy, Clone)]
struct Particle;

#[derive(Debug, Copy, Clone)]
struct Index(usize);

pub struct ParticlesSystem{
    last_position: (Vec2,f32),
    index: usize
}

impl ParticlesSystem{

    pub fn new(world: &mut World) -> ParticlesSystem{
        for i in 0..MAX_PARTICLES{
            world.spawn((
                Particle,
                Index(i),
                Position(Vec2::ZERO),
                Direction(Vec2::ZERO),
                Speed(PARTICLE_SPEED),
                Life(0.0),
            ));
        }

        ParticlesSystem{
            last_position: (Vec2::ZERO, 0.0),
            index: 0
        }
    }
    
    pub fn movement(&mut self, world: &mut World){
        world.query::<(&Position, &Rotation)>()
        .with::<&Player>()
        .iter().for_each(|(pos, rot)|{
            self.last_position = (pos.0, rot.0);
        });

        world.query_mut::<(&Index, &mut Position, &mut Direction, &Speed, &mut Life)>()
        .with::<&Particle>()
        .into_iter()
        .for_each(|(index, pos, dir, speed, life)|{

            if life.0 <= 0.0 && index.0 == self.index{
                pos.0 = self.last_position.0;
                life.0 = PARTICLE_LIFE;
                
            }

            if life.0 > 0.0 {
                life.0 -= 0.5;
                draw_circle(pos.0.x, pos.0.y, 10.0, RED);
            }

        });
        if self.index == MAX_PARTICLES-1{
            self.index = 0
        }else{
            self.index += 1
        }
    }
}