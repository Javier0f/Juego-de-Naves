use macroquad::prelude::*;
use macroquad::ui;
use hecs::{World, Entity};

mod player;
mod collision;
mod asteroids;
mod components;

use player::*;
use collision::*;
use asteroids::*;
use components::*;

struct Player;

#[derive(Debug)]
struct Asteriod;

#[derive(Debug)]
struct PartAsteroid;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position{
    x: f32,
    y: f32,
}

#[derive(Debug,Copy, Clone)]
struct Particle{
    pos: Vec2,
    rot: Vec2,
    vel: f32,
    life: f32,
    size: f32,
    color: Color,
}

#[derive(Debug,Copy,Clone)]
struct Bullet{
    pos: Vec2,
    rot: Vec2,
    vel: f32,
    life:f32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity(Vec2);
#[derive(Debug)]
struct Rot(f32);
#[derive(Debug)]
struct Life(u8);


#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let _background = Color::new(0.01, 0.01, 0.05, 1.0);

    let _bullet_color = Color::new(0.9, 0.8, 0.2, 1.0);

    const MAX_LIFE: u8 = 100;

    let color_player_1 = Color::new(0.9 , 0.1 , 0.4 , 1.0);
    let color_player_2 = Color::new(0.9 , 0.9 , 0.1 ,1.0);
    let mut lifebar_color = GREEN;
    let player = world.spawn((
        Player,
        Position{x: screen_width() / 2.0, y: screen_height() / 2.0},
        Size(30.0),
        Speed(5.0),
        Velocity(Vec2::ZERO),
        Rot(0.0),
        Life(100),
    ));

    let mut collision_system = CollisionSystem::new();

    add_player(&mut world);

    let color_asteroid_2 = Color::new(0.2, 0.4, 0.95, 1.0);
    for _i in 0..30 {
        add_asteroid(&mut world);
    }

    let render_target = render_target(480,270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    const MAX_PARTICLES : usize = 100;
    let mut p_index:usize = 0;

    const MAX_BULLET : usize = 10;
    let mut b_index: usize = 0;
    let mut b_cooldown = 0.0;
    let mut b_collide = 0;

    let mut entities_collide: Vec<(Entity, Vec2, Vec2)> = Vec::with_capacity(26);

    let mut bullet_vec = [Bullet{
        pos: Vec2::ZERO,
        rot: Vec2::ZERO,
        vel: 0.0,
        life: 0.0,
    }; MAX_BULLET];

    let particle_color = Color::new(1.0, 0.3, 0.1, 1.0);
    let mut particles = [Particle{
        pos: Vec2::ZERO,
        rot: Vec2::ZERO,
        vel: 0.0,
        life:0.0,
        size: 0.0,
        color: particle_color
    }; MAX_PARTICLES];

    let mut asteroid_pos_vec = Vec2::ZERO;
    let mut destroid_asteroid = [world.spawn(()); MAX_BULLET / 2];

    let mut last_position: Position = Position{x:0.0, y:0.0};

    loop{
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(_background);

        
        // <----- MOVEMENT AND COLLISION -----> //

        {// <-------- COLLISION --------> //
            let mut query = world.query::<(Entity, &Position, &Velocity, &Size, &Speed)>();
            let mut asteroid_2_collide: Vec<_> = query.iter().collect::<Vec<_>>();

            bullet_vec.iter_mut().for_each(|b|{
                asteroid_2_collide.iter_mut().for_each(|a|{
                    if b.life > 0.0 {
                        asteroid_pos_vec = vec2(a.1.x, a.1.y);
                        let distance = b.pos.distance(asteroid_pos_vec);

                        if distance < a.3.0 && a.0 != player{
                            b.life = 0.0;
                            destroid_asteroid[b_collide] = a.0;
                            b_collide += 1;
                            if b_collide >= MAX_BULLET / 2 { b_collide = 0}
                        }
                    }
                });
            });

            for a in asteroid_2_collide.iter(){
                for b in asteroid_2_collide.iter(){
                    if a.0 != b.0{
                        let vec_a = Vec2::new(a.1.x, a.1.y);
                        let vec_b = Vec2::new(b.1.x, b.1.y);

                        let distance = vec_a.distance(vec_b);
                        let suma_radios = a.3.0 + b.3.0;

                        if distance < suma_radios && distance > 0.0 {
                            let repulcion = (vec_a - vec_b).normalize();
                            let superposicion = suma_radios - distance;

                            let new_pos = Vec2::new(
                                vec_a.x + repulcion.x * (superposicion * 0.5),
                                vec_a.y + repulcion.y * (superposicion * 0.5)
                            );

                            entities_collide.push((a.0, repulcion, new_pos));
                        }
                    }
                }
            }
        }

        player_movement(&mut world, dt);
        asteroid_movement(&mut world, dt);
        collision_system.process(&world);
        collision_system.collision_check(&mut world);

        world.query_mut::<(Entity, &PartAsteroid, &mut Position, &mut Rot, &mut Velocity, &Speed, &Size)>().into_iter().for_each(|p|{
            let entity = p.0;
            let pos = p.2;
            let rot = p.3;
            let vel = p.4;
            let speed = p.5;
            let size = p.6;

            entities_collide.iter().for_each(|collide| {
                if collide.0 == entity{
                    pos.x = collide.2.x;
                    pos.y = collide.2.y;
                    vel.0 = collide.1
                }
            });

            pos.x = pos.x + vel.0.x * speed.0 * dt;
            pos.y = pos.y + vel.0.y * speed.0 * dt;

            rot.0 = rot.0 + dt * speed.0;

            if pos.x < size.0 * -1.{
                pos.x = 1920.0 + size.0;
            }
            if pos.x > (1920.0 + size.0) {
                pos.x = -1. * size.0
            }

            if pos.y < size.0 * -1.{
                pos.y = 1080.0 + size.0;
            }
            if pos.y > (1080.0 + size.0){
                pos.y = -1. * size.0
            }
        });

        // <-------- RENDER --------> //

        bullet_vec.iter_mut().for_each(|b|{
            if b.life > 0.0 {
                b.pos.x = b.pos.x + b.rot.x * b.vel;
                b.pos.y = b.pos.y + b.rot.y * b.vel;
                draw_circle(b.pos.x, b.pos.y, 7.0, _bullet_color);
                b.life -= 0.1;
            }
        });

        particles.iter_mut().for_each(|p|{
            if p.life > 0.0 {
                p.pos.x = p.pos.x + p.rot.x * p.vel * dt;
                p.pos.y = p.pos.y + p.rot.y * p.vel * dt;
                p.color.a -= 0.03;
                p.life = p.life - 0.1;
                p.size = p.size - 0.8;
                if p.life < 1.5 / 2.0 {p.color.g += 0.3 ;p.color.b += 0.5; p.color.r -= 0.1}
                draw_circle(p.pos.x, p.pos.y, p.size, p.color);
            }
        });

        for (_type, pos, size, rot) in world.query::<(&Player, &Position, &Size, &Rot)>().iter() {
            let sen_a = rot.0.to_radians().sin();
            let cos_a = rot.0.to_radians().cos();

            let h = size.0 ;
            let b = size.0 * 1.3;

            let vec_a = vec2(
                pos.x + h * sen_a,
                pos.y - h * cos_a
            );

            let vec_b = vec2(
                pos.x - cos_a * b - sen_a * h,
                pos.y - sen_a * b + cos_a * h
            );


            let vec_c = vec2(
                pos.x + cos_a * b - sen_a * h,
                pos.y + sen_a * b + cos_a * h
            );

            draw_poly_lines(pos.x, pos.y, 3, size.0 + 3.0, rot.0 + 30.0 , 14.0, color_player_1);
            draw_triangle(vec_a, vec_b, vec_c, color_player_2);
        }

        for (_type, pos, rot, life) in world.query::<(&Player, &Position, &Rot, &Life)>().iter(){

            let cos = rot.0.to_radians().cos();
            let sen = rot.0.to_radians().sin();

            let dis = 40.0;
            
            let vec_1 = vec2(pos.x + (cos - sen) * dis, pos.y + (sen + cos) * dis);
            let vec_2 = vec2(pos.x + (-dis * cos - dis * sen), pos.y + (-dis * sen + dis * cos));

            let f = (life.0 as f32) / (MAX_LIFE as f32);

            let vec_c = vec2((vec_1.x + vec_2.x) / 2.0, (vec_1.y + vec_2.y) / 2.0);

            let vec_a = vec_c + f * (vec_1 - vec_c);
            let vec_b = vec_c + f * (vec_2 - vec_c);

            if life.0 < 75 {
                lifebar_color = YELLOW
            }
            if life.0 < 50{
                lifebar_color = ORANGE
            }
            if life.0 < 25{
                lifebar_color = RED
            }

            draw_line(vec_a.x, vec_a.y, vec_b.x, vec_b.y, 10.0 as f32, lifebar_color);
        }

        world.query::<(&PartAsteroid, &Position, &Size, &Sides, &Rot)>().iter().for_each(|a|{
            draw_poly_lines(a.1.x, a.1.y, a.3.0, a.2.0, a.4.0, 4.5, color_asteroid_2)
        });

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams{
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        destroid_asteroid.iter().for_each(|a| {
            
            world.query_mut::<(Entity, &Asteriod, &Position)>().into_iter().for_each(|ast|{
                if ast.0 == *a {
                    last_position = *ast.2;
                }
            });
            
            if let Ok(_) = world.despawn(*a) && last_position != (Position{x:0.0, y: 0.0}) {
                for _i in 0..3{
                    let size_rand = rand::gen_range(20.0, 30.0);
                    let rand_sides = rand::gen_range(3, 7);
    
                    let rand_vel_x = rand::gen_range(-1.0, 1.0);
                    let rand_vel_y = rand::gen_range(-1.0, 1.0);
    
                    let rand_rot =  rand::gen_range(30.0, 35.0);
    
                    world.spawn((
                        PartAsteroid,
                        Size(size_rand),
                        Sides(rand_sides),
                        last_position,
                        Velocity(Vec2::new(rand_vel_x, rand_vel_y)),
                        Speed(80.0),
                        Rot(rand_rot),
                    ));
                }

                last_position = Position{x:0.0, y:0.0};
            }
        });

        // entities_collide.clear();
        collision_system.clear();
        next_frame().await;
        // return
    }
}