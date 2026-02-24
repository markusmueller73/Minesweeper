use crate::game::{board::{Board, BoardSize}, config::Config, constants::{ BOARD_PADDING, CELL_SIZE_BOARD_S, MAX_CELLS_BOARD_S, MENUBAR_HEIGHT, SCOREBOARD_HEIGHT}};
use crate::engine::{asset_manager, event_manager};
use egor::math::{Vec2, vec2};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum GameMode {
    // during a running game
    Game(BoardSize),
    // after selecting a game, the initialization is made here
    GameInit(BoardSize),
    // default, nothing happens, waiting for player selection
    #[default]
    GameStart,
    // the game was lost
    GameLost,
    // wait for player action to start the new game
    GameWaitForPlayer,
    // ...
    GameWon,
    // player select to quit the game
    QuitGame
}

#[derive(Debug, Default)]
pub struct GameState {
    // Window settings
    pub config: Config,
    pub resized: bool,
    pub focused: bool,
    // Resources
    pub assets: asset_manager::Assets,
    // Events
    pub mouse: event_manager::MouseState,
    // Game specific
    pub mode: GameMode,
    pub board: Board,
    pub board_size: BoardSize,
    pub board_size_f: f32,
    pub board_offset: Vec2,
    pub cell_size: f32,
    pub score: u32,
    pub time_elapsed: u32,
    pub paused: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            focused: true,
            board: Board::new(BoardSize::Small, false),
            board_size: BoardSize::Small,
            board_size_f: CELL_SIZE_BOARD_S * MAX_CELLS_BOARD_S,
            board_offset: vec2(BOARD_PADDING, BOARD_PADDING + SCOREBOARD_HEIGHT + MENUBAR_HEIGHT),
            cell_size: CELL_SIZE_BOARD_S,
            paused: true,
            ..Default::default()
        }
    }
}
