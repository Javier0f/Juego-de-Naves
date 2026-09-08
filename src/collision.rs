use hecs::{
    World,
    Entity,
    Without
};
use macroquad::{
    math::*, 
};

use crate::components::*;

pub struct CollisionSystem{
    collision: Vec<(Entity, Vec2, Vec2)>
}

impl CollisionSystem{
    pub fn new() -> CollisionSystem{
        CollisionSystem{
            collision: vec![],
        }
    }

    pub fn process(&mut self, world: &World){
        // world.query::<(Entity, &Position, &Size)>()
        world.query::<Without<(Entity, &Position, &Size), &Particle>>()
        .iter()
        .for_each(|(enti_a, pos_a, size_a)|{
            // world.query::<(Entity, &Position, &Size)>()
            world.query::<Without<(Entity, &Position, &Size), &Particle>>()
            .iter()
            .for_each(|(_enti_b, pos_b, size_b)|{
                let distance = pos_a.0.distance(pos_b.0);
                let sum_radios = size_a.0 + size_b.0;

                if distance < sum_radios && distance > 0.0{
                        
                let repulsion = (pos_a.0 - pos_b.0).normalize();
                let new_pos = vec2(
                    pos_a.0.x + repulsion.x * ((sum_radios - distance) * 0.5),
                    pos_a.0.y + repulsion.y * ((sum_radios - distance) * 0.5),
                );

                self.collision.push((enti_a, repulsion, new_pos));
                }
            });
        });
    }
    
    pub fn collision_check(&self, world: &mut World){

        world.query_mut::<(Entity, &mut Position, &mut Direction)>().into_iter().for_each(|(enti_a, pos_a, dir_a)|{
            self.collision.iter().for_each(|(enti, rep, pos)|{
                if *enti == enti_a {
                    pos_a.0.x = pos.x;
                    pos_a.0.y = pos.y;
                    dir_a.0 = *rep;
                }
            })
        })
    }

    pub fn clear(&mut self){
        self.collision.clear();
    }
}