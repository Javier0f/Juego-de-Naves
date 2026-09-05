
use hecs::World;
use macroquad::{
    math::*, 
    color::*,
    shapes::*
};

use crate::components::*;

const MAX_PARTICLES: usize = 20;

#[derive(Debug, Copy, Clone)]
struct Particle;

pub struct ParticlesSystem{
    index: usize,
    particles: [Position;MAX_PARTICLES]
}

impl ParticlesSystem{
    pub fn new() -> ParticlesSystem{
        ParticlesSystem{
            index: 0,
            particles: [Position(Vec2::ZERO); 20]
        }
    }

    pub fn query_player_position(&mut self, world: &World){
        world.query::<(&Player, &Position)>().iter().for_each(|(_type, pos)|{
            
            self.particles[self.index] = *pos;
            
            if self.index >= MAX_PARTICLES -1 {
                self.index = 0;
            }else{
                self.index += 1;
            }

        })
    }

    pub fn process(&self){
        self.particles.iter().for_each(|pos|{
            draw_circle(pos.0.x, pos.0.y, 5.0, RED);
        })
    }
}