use crate::game::constants::{APP_NAME, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

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
        Config {
            title: APP_NAME.to_string(),
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            resizeable: false,
            vsync: true,
        }
    }
}
