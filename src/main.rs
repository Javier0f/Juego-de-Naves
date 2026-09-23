use macroquad::{
    prelude::*,
    ui::*,
    text::load_ttf_font,
};
use hecs::World;

mod player;
mod collision;
mod asteroids;
mod particles;
mod bullets;
mod lifebar;
mod game_state;
mod menu;
mod components;

use player::*;
use collision::*;
use asteroids::*;
use particles::*;
use bullets::*;
use lifebar::*;
use game_state::*;
use menu::*;

#[macroquad::main("Nave")]
async fn main() {
    let mut world = World::new();

    let _background = Color::new(0.01, 0.01, 0.05, 1.0);

    let mut particle_system = ParticlesSystem::new(&mut world);

    let mut game = Game::new();

    let mut cooldown: u8 = 0;

    add_player(&mut world);
    add_asteroid(&mut world, 30);

    let render_target = render_target(480,270);

    render_target.texture.set_filter(FilterMode::Nearest);

    let mut retro_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0));
    retro_camera.render_target = Some(render_target.clone());

    // ESTILO DE LA INTERFAZ
    let skin1 = {
        let font = load_ttf_font("./font/press-start-2p-latin-400-normal.ttf")
        .await
        .unwrap();

        let label_style = root_ui()
        .style_builder()
        .with_font(&font)
        .unwrap()
        .text_color(Color::from_rgba(40, 40, 26, 255))
        .font_size(30)
        .build();

        let button_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(180, 180, 120, 255))
        .with_font(&font)
        .unwrap()
        .build();

        Skin {
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };

    let win1 = skin1.clone();

    root_ui().push_skin(&win1);

    loop{
        let dt = get_frame_time();
        set_camera(&retro_camera);
        clear_background(_background);

        match game.state {
            GameState::RUN =>{
                particle_system.movement(&mut world, dt);
                bullets(&mut world, dt, &mut cooldown);
                inputs_player(&mut world, dt);
                render(&mut world);
                movement(&mut world, dt);
                collision_system(&mut world);
                lifebar(&mut world, &mut game);
            },
            GameState::MENU => {
                main_menu(&mut game);
            },
            GameState::END => {
                end(&mut game)
            }
            _ => ()
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

        next_frame().await;
    }
}
