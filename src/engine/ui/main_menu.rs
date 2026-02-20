use crate::game::constants::{DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT};
use crate::engine::ui::{MENU_BUTTON_HEIGHT, MENU_BUTTON_WIDTH, rectangle::Rectangle};

#[derive(Clone, Debug)]
pub struct MainMenu {
    pub button_rect: Vec<Rectangle>
}

impl Default for MainMenu {
    fn default() -> Self {
        MainMenu::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32)
    }
}

impl MainMenu {
    pub fn new(window_width: f32, _window_height: f32) -> MainMenu {
        let mut rect_vec: Vec<Rectangle> = Vec::with_capacity(4);
        for i in 0..4 {
            let x = (window_width - MENU_BUTTON_WIDTH) / 2.0;
            let y = 100.0 + i as f32 * 1.5 * MENU_BUTTON_HEIGHT;
            rect_vec.push(Rectangle::new(x, y, MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT));
        }
        MainMenu {
            button_rect: rect_vec,
        }
    }
    pub fn get_button_at_pos(&self, x: f32, y: f32) -> usize {
        let mut result: usize = 0;
        for r in 0..self.button_rect.len() {
            result += 1;
            if self.button_rect[r].is_inside(x, y) {
                return result;
            }
        }
        0
    }
}
