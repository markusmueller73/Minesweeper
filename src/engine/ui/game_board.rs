use crate::game::GameState;
use crate::engine::ui::{MENU_BUTTON_HEIGHT, MENU_BUTTON_WIDTH};
use egor::{
    math::vec2,
    render::{Color, Graphics},
};
use std::collections::HashMap;

pub fn draw_playfield(graphics: &mut Graphics, game_state: &GameState) {

    // Calculate the offsets for the playfield
    let cols = game_state.board.get_width();
    let rows = game_state.board.get_height();

    let cell_size = game_state.cell_size;
    let x_offset = ((game_state.window_size.x as f32) - cell_size * cols as f32 ) / 2.0;
    let y_offset = (game_state.window_size.y as f32) - cell_size * rows as f32 - 10.0;

    // Clear screen
    graphics.clear(Color::new([0.2,0.2,0.2,1.0]));

    // Draw game title


    // Draw scores and timer
    let (score_tex1,score_tex2,score_tex3) = get_textures_of_number(game_state.score, &game_state.texture);
    let (time_tex1,time_tex2,time_tex3) = get_textures_of_number(game_state.time_elapsed, &game_state.texture);

    graphics.rect()
        .at(vec2(x_offset, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(score_tex1);
    graphics.rect()
        .at(vec2(x_offset + 25.0, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(score_tex2);
    graphics.rect()
        .at(vec2(x_offset + 50.0, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(score_tex3);

    graphics.rect()
        .at(vec2( as f32 - x_offset - 75.0, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(time_tex1);
    graphics.rect()
        .at(vec2(WINDOW_WIDTH as f32 - x_offset - 50.0, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(time_tex2);
    graphics.rect()
        .at(vec2(WINDOW_WIDTH as f32 - x_offset - 25.0, y_offset - 50.0))
        .size(vec2(20.0, 45.0))
        .texture(time_tex3);

    // Draw the playfield
    for y in 0..rows {
        for x in 0..cols {

            // Get the cell content
            let (marker, is_revealed, bombs_around, is_bomb) = game_state.board.get_cell(x as usize, y as usize);

            // Texture Layer 1 for the cell
            let layer1: isize = if is_revealed {
                *game_state.texture.get("field_revealed").unwrap()
            } else {
                *game_state.texture.get("field").unwrap()
            };

            // Texture Layer 2 for the cell
            let layer2 = if is_revealed {
                match bombs_around {
                    1 => *game_state.texture.get("text_nb1").unwrap(),
                    2 => *game_state.texture.get("text_nb2").unwrap(),
                    3 => *game_state.texture.get("text_nb3").unwrap(),
                    4 => *game_state.texture.get("text_nb4").unwrap(),
                    5 => *game_state.texture.get("text_nb5").unwrap(),
                    6 => *game_state.texture.get("text_nb6").unwrap(),
                    7 => *game_state.texture.get("text_nb7").unwrap(),
                    8 => *game_state.texture.get("text_nb8").unwrap(),
                    _ => {
                        if is_bomb {
                            *game_state.texture.get("bomb").unwrap()
                        } else {
                            *game_state.texture.get("field_revealed").unwrap()
                        }
                    }
                }
            } else {
                match marker {
                    CellMarker::HasBomb => *game_state.texture.get("flag").unwrap(),
                    CellMarker::GuessBomb => *game_state.texture.get("text_qmark").unwrap(),
                    CellMarker::None => *game_state.texture.get("field").unwrap(),
                }
            };

            // Draw the layers
            graphics.rect()
                .at(vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset))
                .size(vec2(cell_size, cell_size))
                .texture(layer1 as usize);

            if layer2 >= 0 && layer2 != layer1 {
                graphics.rect()
                    .at(vec2(x as f32 * cell_size + x_offset, y as f32 * cell_size + y_offset))
                    .size(vec2(cell_size, cell_size))
                    .texture(layer2 as usize);
            }

        } //for x in 0..cols
    } //for y in 0..rows

}

fn get_textures_of_number(value: u32, texture: &HashMap<String,isize>) -> (usize,usize,usize) {
    let mut number: usize = value as usize;
    let hundred: usize = number / 100;
    number -= hundred * 100;
    let tenner: usize = number / 10;
    number -= tenner * 10;
    let tex1 = match hundred {
        1 => *texture.get("digit_1").unwrap(),
        2 => *texture.get("digit_2").unwrap(),
        3 => *texture.get("digit_3").unwrap(),
        4 => *texture.get("digit_4").unwrap(),
        5 => *texture.get("digit_5").unwrap(),
        6 => *texture.get("digit_6").unwrap(),
        7 => *texture.get("digit_7").unwrap(),
        8 => *texture.get("digit_8").unwrap(),
        9 => *texture.get("digit_9").unwrap(),
        _ => *texture.get("digit_0").unwrap(),
    };
    let tex2 = match tenner {
        1 => *texture.get("digit_1").unwrap(),
        2 => *texture.get("digit_2").unwrap(),
        3 => *texture.get("digit_3").unwrap(),
        4 => *texture.get("digit_4").unwrap(),
        5 => *texture.get("digit_5").unwrap(),
        6 => *texture.get("digit_6").unwrap(),
        7 => *texture.get("digit_7").unwrap(),
        8 => *texture.get("digit_8").unwrap(),
        9 => *texture.get("digit_9").unwrap(),
        _ => *texture.get("digit_0").unwrap(),
    };
    let tex3 = match number {
        1 => *texture.get("digit_1").unwrap(),
        2 => *texture.get("digit_2").unwrap(),
        3 => *texture.get("digit_3").unwrap(),
        4 => *texture.get("digit_4").unwrap(),
        5 => *texture.get("digit_5").unwrap(),
        6 => *texture.get("digit_6").unwrap(),
        7 => *texture.get("digit_7").unwrap(),
        8 => *texture.get("digit_8").unwrap(),
        9 => *texture.get("digit_9").unwrap(),
        _ => *texture.get("digit_0").unwrap(),
    };
    (tex1 as usize,tex2 as usize,tex3 as usize)
}
