use egor::render::Color;

pub const APP_NAME: &str = "Minesweeper";

pub const MENUBAR_HEIGHT: f32 = 28.0;

pub const SCOREBOARD_HEIGHT: f32 = 80.0;
pub const BOARD_PADDING: f32 = 10.0;

pub const MAX_CELLS_BOARD_S: f32 = 10.0;
pub const MAX_CELLS_BOARD_M: f32 = 20.0;
pub const MAX_CELLS_BOARD_L: f32 = 30.0;

pub const CELL_SIZE_BOARD_S: f32 = 40.0;
pub const CELL_SIZE_BOARD_M: f32 = 35.0;
pub const CELL_SIZE_BOARD_L: f32 = 25.0;

pub const WINDOW_BG_COLOR: Color = Color::new([0.75, 0.75, 0.75, 1.0]);
pub const BOARD_BG_COLOR: Color = Color::new([0.3, 0.3, 0.3, 1.0]);
pub const SCORE_BG_COLOR_1: Color = Color::new([0.2, 0.2, 0.2, 1.0]);
pub const SCORE_BG_COLOR_2: Color = Color::new([0.1, 0.1, 0.1, 1.0]);
