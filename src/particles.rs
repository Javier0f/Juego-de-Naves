
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
        let a = [(
            Particle,
            Position(Vec2::ZERO),
            Direction(Vec2::ZERO),
            Life(0),
            Speed(0.0),
        ); 20];

        ParticlesSystem{
            particles: a 
        }
    }
}