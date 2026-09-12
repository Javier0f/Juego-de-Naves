
use hecs::{World, With};
use macroquad::{
    math::*, 
    color::*,
    shapes::*,
    rand
};

use crate::components::*;

const MAX_PARTICLES: usize = 50;
const PARTICLE_LIFE: i8 = 25;
const PARTICLE_SPEED: f32 = 200.0;
const PARTICLE_SIZE: f32 = 25.0;
const DEFAULT_COLOR: (f32,f32,f32,f32) = (1.9, 0.3, 0.3, 1.0);

#[derive(Debug, Copy, Clone)]
struct PColor(f32,f32,f32,f32);

#[derive(Debug, Copy, Clone)]
struct Index(usize);

pub struct ParticlesSystem{
    last_position: (Vec2,f32),
    index: usize,
    color: Color,
    state: bool,
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
                PColor(1.0, 1.0, 1.0, 1.0),
            ));
        }

        ParticlesSystem{
            last_position: (Vec2::ZERO, 0.0),
            index: 0,
            color: Color::new(0.3, 1.0, 1.0, 1.0),
            state: false,
        }
    }
    
    pub fn movement(&mut self, world: &mut World, dt: f32){
        world.query::<With<(&Position, &Rotation, &OnMove), &Player>>()
        .iter().for_each(|(pos, rot, onmove)|{
            self.last_position = (pos.0, rot.0.to_radians());
            self.state = onmove.0;
        });

        world.query_mut::<With::<(&Index, &mut Position, &mut Direction, &mut Size, &mut Life, &mut PColor), &Particle>>()
        .into_iter()
        .for_each(|(index, pos, dir, size, life, pcolor)|{
            if life.0 > 0 {
                life.0 -= 1;

                self.color.r = pcolor.0;
                self.color.g = pcolor.1;
                self.color.b = pcolor.2;
                self.color.a = pcolor.3;

                pcolor.3 -= 0.03;

                dir.0 = vec2(
                    -self.last_position.1.sin() + rand::gen_range(-1.5, 1.5),
                     self.last_position.1.cos() + rand::gen_range(-1.5, 1.5)
                );

                if life.0 < PARTICLE_LIFE * 4/5 {
                    pcolor.0 -= 0.03;
                    pcolor.1 += 0.4;
                    pcolor.2 += 0.3;
                }

                size.0 -= 1.0;

                pos.0 += dir.0 * PARTICLE_SPEED * dt;

                draw_circle(pos.0.x, pos.0.y, size.0, self.color);
            }
            
            if life.0 <= 0 && index.0 == self.index && self.state{
                size.0 = PARTICLE_SIZE;
                pos.0 = self.last_position.0;
                life.0 = PARTICLE_LIFE;
                pcolor.0 = DEFAULT_COLOR.0;
                pcolor.1 = DEFAULT_COLOR.1;
                pcolor.2 = DEFAULT_COLOR.2;
                pcolor.3 = DEFAULT_COLOR.3;
            }

        });
        if self.index == MAX_PARTICLES-1{
            self.index = 0
        }else{
            self.index += 1
        }
    }
}