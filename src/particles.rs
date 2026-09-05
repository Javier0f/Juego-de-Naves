
use hecs::World;
use macroquad::{
    math::*, 
    color::*,
    shapes::*
};

use crate::components::*;

#[derive(Debug, Copy, Clone)]
struct Particle;

pub struct ParticlesSystem{
    index: u8,
    particles: [(
        Particle,
        Position,
        Direction,
        Life,
        Speed,
    );20]
}

impl ParticlesSystem{
    pub fn new() -> ParticlesSystem{
        ParticlesSystem{
            index: 0,
            particles: [(
                Particle,
                Position(Vec2::ZERO),
                Direction(Vec2::ZERO),
                Life(0),
                Speed(0.0),
            ); 20]
        }
    }
}