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
struct Size(f32);
#[derive(Debug)]
struct Speed(f32);
#[derive(Debug)]
struct Rot(f32);
#[derive(Debug)]
struct Sides(u8);

#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let _background = Color::new(0.01, 0.01, 0.05, 1.0);

    world.spawn((
        Player,
        Position{x: screen_width() / 2.0, y: screen_height() / 2.0},
        Size(30.0),
        Speed(6.0),
        Velocity(Vec2::ZERO),
        Rot(0.0),
    ));

    
    for _i in 0..30 {
        let r = rand::gen_range(0.1, 0.9);
        let g = rand::gen_range(0.1, 0.9);
        let b = rand::gen_range(0.1, 0.9);

        let color_asteroid = Color::new(r, g, b, 1.0);


        let size_rand = rand::gen_range(25.0, 90.0);
        let rand_x = rand::gen_range(size_rand, 1920.0 - size_rand);
        let rand_y = rand::gen_range(size_rand, 1080.0 - size_rand);
        let rand_sides = rand::gen_range(5, 10);
        
        let rand_vel_x = rand::gen_range(-1.0, 1.0);
        let rand_vel_y = rand::gen_range(-1.0, 1.0);

        let rand_speed = rand::gen_range(40.0, 150.0);

        let rand_rot =  rand::gen_range(30.0, 35.0);

        world.spawn((
            Asteriod,
            Size(size_rand),
            Sides(rand_sides),
            Position{x: rand_x, y: rand_y},
            Velocity(Vec2::new(rand_vel_x, rand_vel_y)),
            Speed(rand_speed),
            color_asteroid,
            Rot(rand_rot),
        ));
    }

    let render_target = render_target(480,270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    let mut entities_collide: Vec<(Entity, Vec2, Vec2)> = Vec::with_capacity(26);

    loop{
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(_background);

        // <----- MOVEMENT AND COLLISION -----> //


        {// <-------- COLLISION --------> //
            let mut query = world.query::<(Entity, &Position, &Velocity, &Size, &Speed)>();
            let asteroid_2_collide: Vec<_> = query.iter().collect::<Vec<_>>();

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

        for (enti,_type, pos, speed, vel, rot) in world.query_mut::<(Entity, &Player, &mut Position, &Speed, &mut Velocity, &mut Rot)> () {
            let mut dir = Vec2::ZERO;
            let rot_rad = rot.0.to_radians();
            
            if pos.x > 1920.0 + 30.0 {
                pos.x = -30.0
            }
            if pos.x < -30.0{
                pos.x = 1920.0 + 30.0
            }
            if pos.y > 1080.0 + 30.0 {
                pos.y = -30.0
            }
            if pos.y < -30.0{
                pos.y = 1080.0 + 30.0
            }

            entities_collide.iter().for_each(|collide| {
                if collide.0 == enti{
                    pos.x = collide.2.x;
                    pos.y = collide.2.y;
                    vel.0 = collide.1;
                }
            });
    
            if is_key_down(KeyCode::Up)   {
                dir.x = rot_rad.sin();
                dir.y = -rot_rad.cos();
            }
            if is_key_down(KeyCode::Down) {
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
            
            draw_poly_lines(pos.x, pos.y, 3, size.0 + 3.0, rot.0 + 30.0 , 14.0, BLUE);
            draw_triangle(vec_a, vec_b, vec_c, SKYBLUE);
            //draw_circle_lines(pos.x, pos.y, size.0, 1., BLACK); // area de colision
        }

        for (_type, pos, size, sides, rot, color) in world.query_mut::<(&Asteriod, &Position, &Size, &Sides, &mut Rot, &Color)>() {
            // let cos = rot.0.to_radians().cos();
            // let sen = rot.0.to_radians().sin();

            // let dis = size.0 / 2.5;

            // //              ( d , d )
            // let vec_a = vec2( pos.x + dis*cos - dis*sen , pos.y + dis*sen + dis*cos);

            // //              ( d , -d )
            // let vec_b = vec2(pos.x + dis*cos + dis*sen, pos.y + dis*sen - dis*cos);

            // draw_circle_lines(vec_a.x, vec_a.y, 10.0, 4.5, *color);
            // draw_circle_lines(vec_b.x, vec_b.y, 10.0, 4.5, *color);

            draw_poly_lines(pos.x, pos.y, sides.0, size.0, rot.0, 8.5, *color)
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
    }
}
