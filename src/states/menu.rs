use hecs::World;
use macroquad::{
    color::*,
    math::*,
    text::*,
    ui::*,
    window::{screen_height, screen_width},
};

use crate::states::game_state::Game;

use crate::entitys::{asteroids::*, bullets::*, particles::*, player::*};

pub fn main_menu(game: &mut Game) {
    let mid_window = vec2(screen_width() / 2.0, screen_height() / 2.0);

    if root_ui().button(mid_window, "Start") {
        game.switch(1)
    }
}

pub fn end(game: &mut Game, world: &mut World) {
    let mid_window = vec2(screen_width() / 2.0, screen_height() / 2.0);

    draw_text(
        "Game Over",
        screen_width() / 2.0,
        screen_height() / 2.0,
        80.0,
        RED,
    );

    if root_ui().button(mid_window, "[Restart]") {
        world.clear();
        add_player(world);
        add_asteroid(world, 30);
        game.switch(2)
    }
}
