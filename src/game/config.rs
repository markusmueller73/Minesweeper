use crate::game::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub resizeable: bool,
    pub vsync: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizeable: false,
            vsync: true,
        }
    }
}
