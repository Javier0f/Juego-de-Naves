
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

pub struct ParticlesSystem{
    last_position: Vec2
}

impl ParticlesSystem{

    pub fn new(world: &mut World) -> ParticlesSystem{
        for _i in 0..MAX_PARTICLES{
            world.spawn((
                Particle,
                Position(Vec2::ZERO),
                Direction(Vec2::ZERO),
                Speed(PARTICLE_SPEED),
                Life(0.0),
            ));
        }

        ParticlesSystem{
            last_position: Vec2::ZERO
        }
    }
    
    pub fn movement(&mut self, world: &mut World){
        world.query::<&Position>()
        .with::<&Player>()
        .iter().for_each(|pos|{
            self.last_position = pos.0;
        });

        let mut spawn = false;

        world.query_mut::<(Entity, &mut Position, &mut Direction, &Speed, &mut Life)>()
        .with::<&Particle>()
        .into_iter()
        .for_each(|(id ,pos, dir, speed, life)|{
            if life.0 <= 0.0 {
                if !spawn {
                    pos.0 = self.last_position;
                    life.0 = PARTICLE_LIFE;
                    spawn = true;
                }
            }else{
                life.0 -= 1.0;
            }
        });

        world.query::<(&Position, &Life)>()
        .with::<&Particle>()
        .iter()
        .for_each(|(pos, life)|{
            if life.0 > 0.0 {
                draw_circle(pos.0.x, pos.0.y, 5.0, RED);
            }
        })
    }
}