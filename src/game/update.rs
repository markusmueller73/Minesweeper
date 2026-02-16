use crate::game::{
    CELL_SIZE_LARGE_BOARD,
    CELL_SIZE_MED_BOARD,
    CELL_SIZE_SMALL_BOARD,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    GameMode,
    GameState,
};
use crate::minesweeper::board::BoardSize;

pub fn update(game_state: &mut GameState) {
    match game_state.mode {
        // Handle main menu logic
        GameMode::MainMenu => {
            // game_state.mouse.left = left mousebutton was released
            if game_state.mouse.left {

                // check if the mouse position is within the bounds of the main menu buttons
                game_state.mode = game_state.main_menu.get_selected_button(game_state.mouse.position);

                match game_state.mode {

                    GameMode::Game(board_size) => {
                        match board_size {
                            BoardSize::Small => {
                                game_state.board.set_new_board(BoardSize::Small);
                                game_state.cell_size = CELL_SIZE_SMALL_BOARD;
                            },
                            BoardSize::Medium => {
                                game_state.board.set_new_board(BoardSize::Medium);
                                game_state.cell_size = CELL_SIZE_MED_BOARD;
                            },
                            BoardSize::Large => {
                                game_state.board.set_new_board(BoardSize::Large);
                                game_state.cell_size = CELL_SIZE_LARGE_BOARD;
                            },
                        }
                        game_state.board_offset.x = ((WINDOW_WIDTH as f32) - game_state.cell_size * game_state.board.get_width() as f32 ) / 2.0;
                        game_state.board_offset.y = (WINDOW_HEIGHT as f32) - game_state.cell_size * game_state.board.get_height() as f32 - 10.0;
                        game_state.score = 0;
                        game_state.time_elapsed = 0;
                        game_state.paused = false;
                    },

                    GameMode::ExitGame => {
                        game_state.mode = GameMode::ExitGame;
                    },

                    _ => (),

                };
            }
        }
        // Handle game logic
        GameMode::Game(_board_size) => {
            let cell_x = (game_state.mouse.position.x - game_state.board_offset.x) / game_state.cell_size;
            let cell_y = (game_state.mouse.position.y - game_state.board_offset.y) / game_state.cell_size;
            if game_state.mouse.left {
                if game_state.board.pick_cell(cell_x as usize, cell_y as usize) {
                    game_state.mode = GameMode::GameLost;
                } else if game_state.board.check_win_condition() {
                    game_state.mode = GameMode::GameWon;
                }
            }
            if game_state.mouse.right {
                game_state.board.mark_cell(cell_x as usize, cell_y as usize);
                game_state.score = game_state.board.check_correct_flagged_bombs();
            }

        }
        // Handle game won logic
        GameMode::GameWon => {
            game_state.paused = true;
            if game_state.mouse.left {
                game_state.mode = GameMode::MainMenu;
            }
        }
        // Handle game lost logic
        GameMode::GameLost => {
            game_state.paused = true;
            if game_state.mouse.left {
                game_state.mode = GameMode::MainMenu;
            }
        }
        _ => ()
    }
}
