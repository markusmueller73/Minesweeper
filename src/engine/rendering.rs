use crate::game::{cell::CellMarker, constants::*, game_state::{GameMode, GameState}};
use egor::{app::egui::TextBuffer, math::{Rect, vec2}, render::{Align, Color, Graphics}};

pub fn render_scene(graphics: &mut Graphics, game_state: &GameState) {

    let score = format!("{:03}", game_state.score);
    let timer = format!("{:03}", game_state.time_elapsed);
    let sb_size = 48.0;

    // clear background
    graphics.clear(WINDOW_BG_COLOR);
    graphics.text("")
        .in_rect(Rect::new(vec2(0.0, 0.0), vec2(game_state.config.width as f32, game_state.config.height as f32)), Align::MiddleCenter)
        .color(WINDOW_BG_COLOR);

    // draw backgrd for the scores
    graphics.rect()
        .at(vec2(5.0, 5.0 + MENUBAR_HEIGHT))
        .size(vec2(150.0, SCOREBOARD_HEIGHT - 10.0))
        .color(SCORE_BG_COLOR_1);
    graphics.rect()
        .at(vec2(10.0, 10.0 + MENUBAR_HEIGHT))
        .size(vec2(140.0, SCOREBOARD_HEIGHT - 20.0))
        .color(SCORE_BG_COLOR_2);
    graphics.text(&score)
        .in_rect(Rect::new(vec2(10.0, 10.0 + MENUBAR_HEIGHT), vec2(140.0, SCOREBOARD_HEIGHT - 20.0)), Align::MiddleCenter)
        .font(game_state.assets.get_font("digital"))
        .size(sb_size)
        .color(Color::new([0.8, 0.8, 0.0, 1.0]));

    // // draw backgrd for the timer
    graphics.rect()
        .at(vec2(game_state.config.width as f32 - 150.0 - 5.0, 5.0 + MENUBAR_HEIGHT))
        .size(vec2(150.0, SCOREBOARD_HEIGHT - 10.0))
        .color(SCORE_BG_COLOR_1);
    graphics.rect()
        .at(vec2(game_state.config.width as f32 - 140.0 - 10.0, 10.0 + MENUBAR_HEIGHT))
        .size(vec2(140.0, SCOREBOARD_HEIGHT - 20.0))
        .color(SCORE_BG_COLOR_2);
    graphics.text(&timer)
        .in_rect(Rect::new(vec2(game_state.config.width as f32 - 140.0 - 10.0, 10.0 + MENUBAR_HEIGHT), vec2(140.0, SCOREBOARD_HEIGHT - 20.0)), Align::MiddleCenter)
        .font(game_state.assets.get_font("digital"))
        .size(sb_size)
        .color(Color::new([0.8, 0.8, 0.0, 1.0]));

    // // draw the board backgrd
    graphics.rect()
        .at(vec2(0.0, SCOREBOARD_HEIGHT + MENUBAR_HEIGHT))
        .size(vec2(game_state.config.width as f32, game_state.config.width as f32))
        .color(BOARD_BG_COLOR);

    match game_state.mode {

        GameMode::GameInit(_board_size) => {
            draw_placeholder_board(graphics, game_state);
        },

        GameMode::Game(_board_size) => {
            draw_board(graphics, game_state);
        },

        GameMode::GameLost => {
            draw_board(graphics, game_state);
            graphics.text(t!("game_lost").as_str())
                .in_rect(Rect::new(
                    vec2(0.0, MENUBAR_HEIGHT + SCOREBOARD_HEIGHT),
                    vec2(game_state.config.width as f32, game_state.config.height as f32 - MENUBAR_HEIGHT - SCOREBOARD_HEIGHT)
                ), Align::MiddleCenter)
                .font(game_state.assets.get_font("mines"))
                .size(20.0)
                .color(Color::RED);
        },

        GameMode::GameStart => {
            draw_placeholder_board(graphics, game_state);
            graphics.text(t!("select_game").as_str())
                .in_rect(Rect::new(
                    vec2(0.0, MENUBAR_HEIGHT + SCOREBOARD_HEIGHT),
                    vec2(game_state.config.width as f32, game_state.config.height as f32 - MENUBAR_HEIGHT - SCOREBOARD_HEIGHT)
                ), Align::MiddleCenter)
                .font(game_state.assets.get_font("mines"))
                .size(20.0)
                .color(Color::GREEN);
        },

        GameMode::GameWaitForPlayer => {
            draw_board(graphics, game_state);
            graphics.text(t!("wait_for_player").as_str())
                .in_rect(Rect::new(
                    vec2(0.0, MENUBAR_HEIGHT + SCOREBOARD_HEIGHT),
                    vec2(game_state.config.width as f32, game_state.config.height as f32 - MENUBAR_HEIGHT - SCOREBOARD_HEIGHT)
                ), Align::MiddleCenter)
                .font(game_state.assets.get_font("mines"))
                .size(20.0)
                .color(Color::BLUE);
        },

        GameMode::GameWon => {
            draw_board(graphics, game_state);
            graphics.text(t!("game_won").as_str())
                .in_rect(Rect::new(
                    vec2(0.0, MENUBAR_HEIGHT + SCOREBOARD_HEIGHT),
                    vec2(game_state.config.width as f32, game_state.config.height as f32 - MENUBAR_HEIGHT - SCOREBOARD_HEIGHT)
                ), Align::MiddleCenter)
                .font(game_state.assets.get_font("mines"))
                .size(20.0)
                .color(Color::GREEN);
        },

        _ => (),
    }

}

fn draw_board(graphics: &mut Graphics, game_state: &GameState) {
    // Calculate the offsets for the playfield
    let cols = game_state.board.get_width();
    let rows = game_state.board.get_height();

    let cell_size = game_state.cell_size;
    let x_offset = game_state.board_offset.x;
    let y_offset = game_state.board_offset.y;

    // Draw the playfield
    for y in 0..rows {
        for x in 0..cols {

            let pos_vec2 = vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset);
            let size_vec2 = vec2(cell_size, cell_size);

            let layer1;
            let mut layer2 = usize::MAX;

            // Get the cell content
            let (marker, is_revealed, bombs_around, is_bomb) = game_state.board.get_cell(x as usize, y as usize);

            // Texture Layer 1 for the cell
            if is_revealed {
                layer1 = game_state.assets.get_texture("field_revealed");
                if is_bomb {
                    layer2 = game_state.assets.get_texture("bomb");
                }
            } else {
                layer1 = game_state.assets.get_texture("field");
                layer2 = match marker {
                    CellMarker::HasBomb => game_state.assets.get_texture("flag"),
                    CellMarker::GuessBomb => game_state.assets.get_texture("qmark"),
                    _ => usize::MAX,
                }
            }

            // Draw the layers
            graphics.rect()
                .at(pos_vec2)
                .size(size_vec2)
                .texture(layer1);

            if layer2 != usize::MAX {
                graphics.rect()
                    .at(pos_vec2)
                    .size(size_vec2)
                    .texture(layer2);
            }

            if bombs_around > 0 && is_revealed {
                let number = format!("{}", bombs_around);
                let color = if bombs_around <= 3 {
                    [1.0, 1.0 - (bombs_around as f32 * 0.3), 0.0, 1.0]
                } else {
                    [1.0 - (bombs_around as f32 * 0.1), 0.0, 0.0, 1.0]
                };
                graphics.text(&number)
                    .in_rect(Rect::new(pos_vec2, size_vec2), Align::MiddleCenter)
                    .font(game_state.assets.get_font("mines"))
                    .size(16.0)
                    .color(Color::new(color));
            }

        } //for x in 0..cols
    } //for y in 0..rows

}

fn draw_placeholder_board(graphics: &mut Graphics, game_state: &GameState) {
    let cols = game_state.board.get_width();
    let rows = game_state.board.get_height();
    let cell_size = game_state.cell_size;
    let x_offset = game_state.board_offset.x;
    let y_offset = game_state.board_offset.y;
    let texture = game_state.assets.get_texture("field");
    for y in 0..rows {
        for x in 0..cols {
            let pos = vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset);
            let size = vec2(cell_size, cell_size);
            graphics.rect()
                .at(pos)
                .size(size)
                .texture(texture);
        }
    }
}
