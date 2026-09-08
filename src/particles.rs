
use hecs::{World, With};//, Entity};
use macroquad::{
    math::*, 
    color::*,
    shapes::*,
    rand
};

use crate::components::*;

const MAX_PARTICLES: usize = 20;
const PARTICLE_LIFE: i8 = 20;
const PARTICLE_SPEED: f32 = 300.0;
const PARTICLE_SIZE: f32 = 5.0;

// #[derive(Debug, Copy, Clone)]
// struct Particle;

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
                Size(PARTICLE_SIZE),
                Life(0),
            ));
        }

        ParticlesSystem{
            last_position: (Vec2::ZERO, 0.0),
            index: 0,
        }
    }
    
    pub fn movement(&mut self, world: &mut World, dt: f32){
        // world.query::<(&Position, &Rotation)>()
        world.query::<With<(&Position, &Rotation), &Player>>()
        // .with::<&Player>()
        .iter().for_each(|(pos, rot)|{
            self.last_position = (pos.0, rot.0.to_radians());
        });

        world.query_mut::<(&Index, &mut Position, &mut Direction, &mut Size, &mut Life)>()
        .with::<&Particle>()
        .into_iter()
        .for_each(|(index, pos, dir, size, life)|{
            if life.0 > 0 {
                life.0 -= 1;

                dir.0 = vec2(
                    -self.last_position.1.sin(),
                    self.last_position.1.cos()
                );

                size.0 += 1.0;

                if life.0 < PARTICLE_LIFE / 2{
                    draw_circle(pos.0.x, pos.0.y, size.0, YELLOW);
                }else{
                    draw_circle(pos.0.x, pos.0.y, size.0, RED);

                }
            }
            
            if life.0 <= 0 && index.0 == self.index{
                size.0 = PARTICLE_SIZE;
                pos.0 = self.last_position.0;
                life.0 = PARTICLE_LIFE;
            }

        });
        if self.index == MAX_PARTICLES-1{
            self.index = 0
        }else{
            self.index += 1
        }
    }
}