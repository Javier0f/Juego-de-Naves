use hecs::World;
use macroquad::{prelude::*, ui::root_ui};

mod collision;
mod components;
mod entitys;
mod lifebar;
mod states;
mod ui_style;

use collision::*;
use entitys::{asteroids::*, bullets::*, particles::*, player::*};
use lifebar::*;
use states::{game_state::*, menu::*};
use ui_style::*;

#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let _background = Color::new(0.01, 0.01, 0.05, 1.0);

    let mut particle_system = ParticlesSystem::new(&mut world);

    let mut game = Game::new();

    let mut cooldown: u8 = 0;

    add_player(&mut world);
    add_asteroid(&mut world, 30);

    let render_target = render_target(480, 270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    let skin1 = ui_skin().await;

    root_ui().push_skin(&skin1);

    loop {
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(_background);

        match game.state {
            GameState::RUN => {
                particle_system.movement(&mut world, dt);
                bullets(&mut world, dt, &mut cooldown);
                inputs_player(&mut world, dt);
                render(&mut world);
                movement(&mut world, dt);
                collision_system(&mut world);
                lifebar(&mut world, &mut game);
            }
            GameState::MENU => {
                main_menu(&mut game);
            }
            GameState::END => {
                end(&mut game, &mut world);
            },
            _ => (),
        }

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
