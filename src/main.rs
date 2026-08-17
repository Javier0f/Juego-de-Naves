use macroquad::prelude::*;
use hecs::{World, Entity};

struct Player;

#[derive(Debug)]
struct Asteriod;

#[derive(Debug, Clone, Copy)]
struct Position{
    x: f32,
    y: f32,
}
#[derive(Debug, Clone, Copy)]
struct Velocity(Vec2);

#[derive(Debug)]
struct EntityType(u8);
#[derive(Debug)]
struct Size(f32);
#[derive(Debug)]
struct Speed(f32);
#[derive(Debug)]
struct Rot(f32);
#[derive(Debug)]
struct Sides(f32);

#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let background = Color::new(0.9, 0.9, 0.9, 1.0);

    world.spawn((
        EntityType(0),
        Player,
        Position{x: screen_width() / 2.0, y: screen_height() / 2.0},
        Size(30.0),
        Speed(3.0),
        Velocity(Vec2::ZERO),
        Rot(22.5),
    ));

    for _i in 0..15 {

        let r = 0.2;// rand::gen_range(0., 1.);
        let g = 0.3;// rand::gen_range(0., 1.);
        let b = 0.6;// rand::gen_range(0., 1.);

        let color = Color::new(r , g , b , 1.0);

        let size_rand = rand::gen_range(55.0, 99.0);
        let rand_x = rand::gen_range(size_rand, 1920.0 - size_rand);
        let rand_y = rand::gen_range(size_rand, 1080.0 - size_rand);
        let rand_sides = rand::gen_range(3.0, 15.0);
        
        let rand_vel_x = rand::gen_range(-1.0, 1.0);
        let rand_vel_y = rand::gen_range(-1.0, 1.0);

        let rand_speed = rand::gen_range(40.0, 150.0);

        let rand_rot =  rand::gen_range(30.0, 35.0);

        world.spawn((
            EntityType(1),
            Asteriod,
            Size(size_rand),
            Sides(rand_sides),
            Position{x: rand_x, y: rand_y},
            Velocity(Vec2::new(rand_vel_x, rand_vel_y)),
            Speed(rand_speed),
            color,
            Rot(rand_rot),
        ));
    }

    let render_target = render_target(480,270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    let mut entities_collide: Vec<(Entity, Vec2, Vec2)> = Vec::with_capacity(26);
    // let mut asteroid_2_collide: Vec<(Entity, EntityType, Position, Velocity, Size, Speed)> = Vec::with_capacity(26);
    // let mut asteroid_2_collide: Vec<_> = Vec::with_capacity(26);

    loop{
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(background);

        // <----- MOVEMENT AND COLLISION -----> //


        {// <-------- COLICIONES DE ASTEROIDES --------> //
            let mut query = world.query::<(Entity ,&EntityType, &Position, &Velocity, &Size, &Speed)>();
            let asteroid_2_collide: Vec<_> = query.iter().collect::<Vec<_>>();

            for a in asteroid_2_collide.iter(){
                for b in asteroid_2_collide.iter(){
                    
                    if a.0 != b.0{
                        let vec_a = Vec2::new(a.2.x, a.2.y);
                        let vec_b = Vec2::new(b.2.x, b.2.y);

                        let distance = vec_a.distance(vec_b);
                        let suma_radios = a.4.0 + b.4.0;

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

        for (enti,_type, pos, speed, vel, rot) in world.query_mut::<(Entity, &Player, &mut Position, &Speed, &mut Velocity, &mut Rot)> () {
            let mut dir = Vec2::ZERO;
            let rot_rad = rot.0.to_radians();

            entities_collide.iter().for_each(|collide| {
                if collide.0 == enti{
                    pos.x = collide.2.x;
                    pos.y = collide.2.y;
                    vel.0 = collide.1;
                }
            });
    
            if is_key_down(KeyCode::Up)   {
                // dir.x += 1.0;
                dir.x = rot_rad.sin();
                dir.y = -rot_rad.cos();
            }
            if is_key_down(KeyCode::Down) {
                // dir.x -= 1.0;
                dir.x = -rot_rad.sin();
                dir.y = rot_rad.cos();
            }
            if is_key_down(KeyCode::Right){rot.0 += 5.0}
            if is_key_down(KeyCode::Left) {rot.0 -= 5.0}


    
            if dir.length_squared() > 0.0 {
                dir = dir.normalize();
            }

            let target = dir * speed.0;

            vel.0 = vel.0.lerp(target, 2.0 * dt);

            pos.x += vel.0.x;
            pos.y += vel.0.y;
        }

        for (entity, _type, pos, rot, vel, speed, size) in world.query_mut::<(Entity, &Asteriod, &mut Position, &mut Rot, &mut Velocity, &Speed, &Size)>(){
            
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
        }


        // <-------- RENDER --------> //

        for (_type, pos, size, rot) in world.query_mut::<(&Player, &Position, &Size, &Rot)>() {
            let rot_sin = rot.0.to_radians().sin();
            let rot_cos = rot.0.to_radians().cos();

            let h = size.0 ;// / 2.;
            let b = size.0 ;// / 3.0;
            
            let vec1 = Vec2::new(
                pos.x + h * rot_sin,
                pos.y - h * rot_cos,
            );
            let vec2 = Vec2::new(
                pos.x - rot_cos * b - rot_sin * h,
                pos.y - rot_sin * b + rot_cos * h,
            );
            let vec3 = Vec2::new(
                pos.x + rot_cos * b - rot_sin * h,
                pos.y + rot_sin * b + rot_cos * h,
            );
            
            draw_triangle(vec1, vec2, vec3, RED);
        }

        for (entity ,_type, pos, size, sides, rot, color) in world.query_mut::<(Entity ,&Asteriod, &Position, &Size, &Sides, &mut Rot, &Color)>() {
            draw_poly_lines(pos.x, pos.y, sides.0 as u8, size.0, rot.0, 8.5, *color)
        }

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

        entities_collide.clear();
        next_frame().await;
        // return;
    }
}
