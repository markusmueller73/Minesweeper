use crate::game::{
    game_state::{GameMode, GameState},
    board::BoardSize,
    constants::*,
};

pub fn update(game_state: &mut GameState) {

    match game_state.mode {

        // Handle game logic
        GameMode::Game(_board_size) => {

            let cell_x = (game_state.mouse.position.x - game_state.board_offset.x) / game_state.cell_size;
            let cell_y = (game_state.mouse.position.y - game_state.board_offset.y) / game_state.cell_size;

            if game_state.mouse.released[0] {
                if game_state.board.pick_cell(cell_x as usize, cell_y as usize) {
                    game_state.mode = GameMode::GameLost;
                } else if game_state.board.check_win_condition() {
                    game_state.mode = GameMode::GameWon;
                }
            }

            if game_state.mouse.released[2] {
                game_state.board.mark_cell(cell_x as usize, cell_y as usize);
                game_state.score = game_state.board.check_correct_flagged_bombs();
            }

        },

        GameMode::GameInit(board_size) => {

            match board_size {

                BoardSize::Small => {
                    game_state.board_size = BoardSize::Small;
                    game_state.board_size_f = CELL_SIZE_BOARD_S * MAX_CELLS_BOARD_S;
                    game_state.cell_size = CELL_SIZE_BOARD_S;
                },

                BoardSize::Medium => {
                    game_state.board_size = BoardSize::Medium;
                    game_state.board_size_f = CELL_SIZE_BOARD_M * MAX_CELLS_BOARD_M;
                    game_state.cell_size = CELL_SIZE_BOARD_S;
                },

                BoardSize::Large => {
                    game_state.board_size = BoardSize::Large;
                    game_state.board_size_f = CELL_SIZE_BOARD_L * MAX_CELLS_BOARD_L;
                    game_state.cell_size = CELL_SIZE_BOARD_S;
                },
            }

            game_state.config.width = (game_state.board_size_f + 2.0 * BOARD_PADDING) as u32;
            game_state.config.height = game_state.config.width + SCOREBOARD_HEIGHT as u32 + MENUBAR_HEIGHT as u32;

            game_state.resized = true;

            game_state.board.set_new_board(game_state.board_size);
            game_state.score = 0;
            game_state.time_elapsed = 0;

            game_state.mode = GameMode::GameWaitForPlayer;
            game_state.paused = true;

        },

        GameMode::GameWaitForPlayer => {
            if game_state.mouse.released[0] {
                game_state.paused = false;
                game_state.mode = GameMode::Game(game_state.board_size);
            }
        },

        // Handle game lost logic
        GameMode::GameLost => {
            game_state.paused = true;
            if game_state.mouse.released[0] {
                game_state.mode = GameMode::GameStart;
            }
        },

        // Handle game won logic
        GameMode::GameWon => {
            game_state.paused = true;
            if game_state.mouse.released[0] {
                game_state.mode = GameMode::GameStart;
            }
        },

        _ => ()
    }
}
