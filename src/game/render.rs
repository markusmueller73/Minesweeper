use crate::game::{GameMode, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::ui::playfield::draw_playfield;
use egor::{
    math::vec2,
    render::{Graphics, Color}
};

pub fn render_scene(graphics: &mut Graphics, game_state: &GameState) {

    graphics.clear(Color::BLACK);

    match game_state.mode {

        GameMode::MainMenu => {
            game_state.main_menu.draw(graphics, game_state.mouse.position);
        },

        GameMode::Game(_board_size) => {
            draw_playfield(graphics, game_state);
        },

        GameMode::GameLost => {
            draw_playfield(graphics, game_state);
            graphics.rect()
                .at(vec2(0.0, 50.0))
                .size(vec2(WINDOW_WIDTH as f32, WINDOW_WIDTH as f32))
                .texture(*game_state.texture.get("kaboom").unwrap() as usize);
        },

        GameMode::GameWon => {
            draw_playfield(graphics, game_state);
            graphics.text("You won!")
                .at(vec2(100.0, WINDOW_HEIGHT as f32 - 100.0))
                .size(50.)
                .color(Color::new([0.8,0.4,0.2,1.0]));
        },

        _ => (),
    }

}
