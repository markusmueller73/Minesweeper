use crate::game::{WINDOW_WIDTH, GameMode};
use crate::minesweeper::board::BoardSize;
use crate::ui::{
    MENU_BUTTON_HEIGHT,
    MENU_BUTTON_WIDTH,
    button::Button,
};
use egor::{
    math::Vec2,
    render::Graphics,
};
use std::collections::HashMap;

pub struct MainMenu {
    logo: Button,
    btn_easy: Button,
    btn_med: Button,
    btn_hard: Button,
    btn_exit: Button,
}

impl Default for MainMenu {
    fn default() -> Self {
        MainMenu::new((WINDOW_WIDTH as f32 - MENU_BUTTON_WIDTH) / 2., 10., MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)
    }
}

impl MainMenu {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {

        let mut y_pos = y;
        let logo = Button::new(x, y, width, height);
        y_pos += height;
        let btn_easy = Button::new(x, y_pos, width, height);
        y_pos += 1.5 * height;
        let btn_med = Button::new(x, y_pos, width, height);
        y_pos += 1.5 * height;
        let btn_hard = Button::new(x, y_pos, width, height);
        y_pos += 2. * height;
        let btn_exit = Button::new(x, y_pos, width, height);

        MainMenu {
            logo,
            btn_easy,
            btn_med,
            btn_hard,
            btn_exit,
        }
    }

    pub fn set_textures(&mut self, texture: &HashMap<String,isize>) {
        self.logo.set_textures(*texture.get("logo").unwrap(), -1, -1);
        self.btn_easy.set_textures(
            *texture.get("button").unwrap(),
            *texture.get("button_hover").unwrap(),
            *texture.get("text_easy").unwrap());
        self.btn_med.set_textures(
            *texture.get("button").unwrap(),
            *texture.get("button_hover").unwrap(),
            *texture.get("text_medium").unwrap());
        self.btn_hard.set_textures(
            *texture.get("button").unwrap(),
            *texture.get("button_hover").unwrap(),
            *texture.get("text_hard").unwrap());
        self.btn_exit.set_textures(
            *texture.get("button").unwrap(),
            *texture.get("button_hover").unwrap(),
            *texture.get("text_quit").unwrap());
}

    pub fn draw(&self, graphics: &mut Graphics, mouse_pos: Vec2) {
        self.logo.draw(graphics, mouse_pos);
        self.btn_easy.draw(graphics, mouse_pos);
        self.btn_med.draw(graphics, mouse_pos);
        self.btn_hard.draw(graphics, mouse_pos);
        self.btn_exit.draw(graphics, mouse_pos);
    }

    pub fn get_selected_button(&self, mouse_pos: Vec2) -> GameMode {
        if self.btn_easy.is_inside(mouse_pos.x, mouse_pos.y) {
            GameMode::Game(BoardSize::Small)
        } else if self.btn_med.is_inside(mouse_pos.x, mouse_pos.y) {
            GameMode::Game(BoardSize::Medium)
        } else if self.btn_hard.is_inside(mouse_pos.x, mouse_pos.y) {
            GameMode::Game(BoardSize::Large)
        } else if self.btn_exit.is_inside(mouse_pos.x, mouse_pos.y) {
            GameMode::ExitGame
        } else {
            GameMode::MainMenu
        }
    }

}
