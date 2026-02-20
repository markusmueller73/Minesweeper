use crate::engine::ui::text_button::TextButton;
use crate::game::{
    GameMode,
    GameState,
    cell::CellMarker,
    constants::{APP_NAME, DEFAULT_WINDOW_WIDTH},
};
use egor::{math::{Rect, vec2}, render::{Graphics, Color}};

pub fn render_scene(graphics: &mut Graphics, game_state: &GameState) {

    match game_state.mode {

        GameMode::MainMenu => draw_main_menu(graphics, game_state),

        GameMode::Game(_board_size) => draw_board(graphics, game_state),

        GameMode::GameLost => {
            draw_board(graphics, game_state);
            graphics.rect()
                .at(vec2(
                    (game_state.config.width - 300) as f32 / 2.0,
                    (game_state.config.height - 200) as f32 / 2.0 - 50.0,))
                .size(vec2(300.0, 200.0))
                .texture(game_state.assets.get_texture("kaboom"));
        },

        GameMode::GameWon => {
            //
        },

        _ => (),
    }

}

fn draw_main_menu(graphics: &mut Graphics, game_state: &GameState) {

    let font = game_state.assets.get_font("mines");
    let def_btn = game_state.assets.get_texture("button");
    let hov_btn = game_state.assets.get_texture("button_hover");

    // Clear screen
    graphics.clear(Color::new([0.3, 0.3, 0.3, 1.0]));

    // game title
    graphics
        .text(APP_NAME)
        .in_rect(Rect::new(vec2(0.0, 0.0), vec2(DEFAULT_WINDOW_WIDTH as f32, 100.)), egor::render::Align::MiddleCenter)
        .size(36.)
        .font(font)
        .color(Color::new([0.9, 0.9, 0.9, 1.0]));

    for i in 0..game_state.main_menu.button_rect.len() {
        // draw the button
        let rect = game_state.main_menu.button_rect[i].clone();
        let btn_font = game_state.assets.get_font("mines");
        let btn_text = match i {
            0 => t!("start_easy_game"),
            1 => t!("start_med_game"),
            2 => t!("start_hard_game"),
            _ => t!("quit_game"),
        };
        let text_color = Color::new([0.9, 0.9, 0.9, 1.0]);
        let btn = TextButton::new(
            rect,
            def_btn,
            hov_btn,
            btn_text.to_string(),
            btn_font,
            text_color
        );
        btn.draw(graphics, game_state.mouse.position);
    }

}

fn draw_board(graphics: &mut Graphics, game_state: &GameState) {
    // Calculate the offsets for the playfield
    let cols = game_state.board.get_width();
    let rows = game_state.board.get_height();

    let cell_size = game_state.cell_size;
    let x_offset = game_state.board_offset.x;
    let y_offset = game_state.board_offset.y;

    // Clear screen
    graphics.clear(Color::new([0.3,0.3,0.3,1.0]));

    // Draw board background
    graphics.rect()
        .at(vec2(0.0, 50.0))
        .size(vec2(game_state.config.width as f32, game_state.config.height as f32 - 50.0))
        .color(Color::new([0.15, 0.15, 0.15, 1.0]));

    // Draw the playfield
    for y in 0..rows {
        for x in 0..cols {

            // Get the cell content
            let (marker, is_revealed, bombs_around, is_bomb) = game_state.board.get_cell(x as usize, y as usize);

            // Texture Layer 1 for the cell
            let layer1: usize = if is_revealed {
                game_state.assets.get_texture("field_revealed")
            } else {
                game_state.assets.get_texture("field")
            };

            // Texture Layer 2 for the cell
            let layer2 = if is_revealed {
                match bombs_around {
                    1 => game_state.assets.get_texture("text_nb1"),
                    2 => game_state.assets.get_texture("text_nb2"),
                    3 => game_state.assets.get_texture("text_nb3"),
                    4 => game_state.assets.get_texture("text_nb4"),
                    5 => game_state.assets.get_texture("text_nb5"),
                    6 => game_state.assets.get_texture("text_nb6"),
                    7 => game_state.assets.get_texture("text_nb7"),
                    8 => game_state.assets.get_texture("text_nb8"),
                    _ => {
                        if is_bomb {
                            game_state.assets.get_texture("bomb")
                        } else {
                            game_state.assets.get_texture("field_revealed")
                        }
                    }
                }
            } else {
                match marker {
                    CellMarker::HasBomb => game_state.assets.get_texture("flag"),
                    CellMarker::GuessBomb => game_state.assets.get_texture("text_qmark"),
                    CellMarker::None => game_state.assets.get_texture("field"),
                }
            };

            // Draw the layers
            graphics.rect()
                .at(vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset))
                .size(vec2(cell_size, cell_size))
                .texture(layer1);

            if layer2 != layer1 {
                graphics.rect()
                    .at(vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset))
                    .size(vec2(cell_size, cell_size))
                    .texture(layer2);
            }

        } //for x in 0..cols
    } //for y in 0..rows

}
