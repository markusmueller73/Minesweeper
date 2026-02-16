use crate::ui::rectangle::Rectangle;
use egor::{
    math::{Vec2, vec2},
    render::Graphics,
};

pub struct Button {
    rect: Rectangle,
    backgrd: isize,
    hover: isize,
    text: isize,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            rect: Rectangle::default(),
            backgrd: -1,
            hover: -1,
            text: -1,
        }
    }
}

impl Button {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Button {
            rect: Rectangle::new(x, y, width, height),
            backgrd: 0,
            hover: 0,
            text: 0,
        }
    }

    pub fn set_textures(&mut self, backgrd: isize, hover: isize, text: isize) {
        self.backgrd = backgrd;
        self.hover = hover;
        self.text = text;
    }

    pub fn is_inside(&self, x: f32, y: f32) -> bool {
        self.rect.is_inside(x, y)
    }

    pub fn draw(&self, graphics: &mut Graphics, mouse_pos: Vec2) {
        if self.is_inside(mouse_pos.x, mouse_pos.y) && self.hover >= 0 {
            graphics.rect()
                .at(vec2(self.rect.x, self.rect.y))
                .size(vec2(self.rect.width, self.rect.height))
                .texture(self.hover as usize);
        } else {
            graphics.rect()
                .at(vec2(self.rect.x, self.rect.y))
                .size(vec2(self.rect.width, self.rect.height))
                .texture(self.backgrd as usize);
        }

        if self.text >= 0 {
            graphics.rect()
                .at(vec2(self.rect.x, self.rect.y))
                .size(vec2(self.rect.width, self.rect.height))
                .texture(self.text as usize);
        }

    }

}
