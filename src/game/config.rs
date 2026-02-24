use crate::game::constants::{APP_NAME, BOARD_PADDING, CELL_SIZE_BOARD_S, MAX_CELLS_BOARD_S, MENUBAR_HEIGHT, SCOREBOARD_HEIGHT};

#[derive(Clone, Debug)]
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizeable: bool,
    pub vsync: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}
impl Config {
    pub fn new() -> Self {
        let width = CELL_SIZE_BOARD_S * MAX_CELLS_BOARD_S + 2.0 * BOARD_PADDING;
        Config {
            title: APP_NAME.to_string(),
            width: width as u32,
            height: (width + SCOREBOARD_HEIGHT + MENUBAR_HEIGHT) as u32,
            resizeable: false,
            vsync: true,
        }
    }
}
