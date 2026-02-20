use crate::game::{
    GameMode,
    GameState,
    board::BoardSize,
    constants::{CELL_SIZE_LARGE_BOARD, CELL_SIZE_MED_BOARD, CELL_SIZE_SMALL_BOARD},
};

pub fn update(game_state: &mut GameState) {

    match game_state.mode {
        // Handle main menu logic
        GameMode::MainMenu => {
            // left mousebutton was released
            if game_state.mouse.released[0] {
                match game_state.main_menu.get_button_at_pos(
                    game_state.mouse.position.x,
                    game_state.mouse.position.y
                ) {
                    1 => {
                        game_state.cell_size = CELL_SIZE_SMALL_BOARD;
                        game_state.mode = GameMode::Game(BoardSize::Small);
                        game_state.resized = true;
                        game_state.config.width = CELL_SIZE_SMALL_BOARD as u32 * 10 + 20;
                        game_state.config.height = game_state.config.width + 50;
                        game_state.board_offset.x = 10.0;
                        game_state.board_offset.y = 60.0;
                        game_state.board.set_new_board(BoardSize::Small);
                    },
                    2 => {
                        game_state.cell_size = CELL_SIZE_MED_BOARD;
                        game_state.mode = GameMode::Game(BoardSize::Medium);
                        game_state.resized = true;
                        game_state.config.width = CELL_SIZE_MED_BOARD as u32 * 20 + 10;
                        game_state.config.height = game_state.config.width + 150;
                        game_state.board_offset.x = 10.0;
                        game_state.board_offset.y = 160.0;
                        game_state.board.set_new_board(BoardSize::Medium);
                    },
                    3 => {
                        game_state.cell_size = CELL_SIZE_LARGE_BOARD;
                        game_state.mode = GameMode::Game(BoardSize::Large);
                        game_state.resized = true;
                        game_state.config.width = CELL_SIZE_LARGE_BOARD as u32 * 30 + 10;
                        game_state.config.height = game_state.config.width + 150;
                        game_state.board_offset.x = 10.0;
                        game_state.board_offset.y = 160.0;
                        game_state.board.set_new_board(BoardSize::Large);
                    },
                    4 => game_state.mode = GameMode::ExitGame,
                    _ => (),
                }
            }
        }
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

        }
        // Handle game won logic
        GameMode::GameWon => {
            game_state.paused = true;
            if game_state.mouse.released[0] {
                game_state.mode = GameMode::MainMenu;
            }
        }
        // Handle game lost logic
        GameMode::GameLost => {
            game_state.paused = true;
            if game_state.mouse.released[0] {
                game_state.mode = GameMode::MainMenu;
            }
        }
        _ => ()
    }
}
