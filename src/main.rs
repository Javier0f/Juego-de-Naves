use macroquad::prelude::*;
use hecs::World;

mod player;
mod collision;
mod asteroids;
mod particles;
mod bullets;
mod components;

use player::*;
use collision::*;
use asteroids::*;
use particles::*;
use bullets::*;

#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let _background = Color::new(0.01, 0.01, 0.05, 1.0);

    let mut particle_system = ParticlesSystem::new(&mut world);

    let mut cooldown: u8 = 0;

    add_player(&mut world);
    add_asteroid(&mut world, 30);

    let render_target = render_target(480,270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    loop{
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(_background);

        particle_system.movement(&mut world, dt);
        bullets(&mut world, dt, &mut cooldown);
        inputs_player(&mut world, dt);
        render(&mut world);
        movement(&mut world, dt);
        collision_system(&mut world);

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

        next_frame().await;
    }
}
