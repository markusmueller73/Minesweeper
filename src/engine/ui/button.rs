use crate::engine::ui::rectangle::Rectangle;
use egor::{
    math::{Vec2, vec2},
    render::Graphics,
};

#[derive(Clone, Debug, Default)]
pub struct Button {
    rect: Rectangle,
    backgrd: usize,
    hover: usize,
}

#[allow(dead_code)]
impl Button {

    pub fn new(rectangle: Rectangle, bg_texture: usize, hover_texture: usize) -> Self {
        Button {
            rect: rectangle,
            backgrd: bg_texture,
            hover: hover_texture,
        }
    }

    pub fn set_textures(&mut self, bg_texture: usize, hover_texture: usize) {
        self.backgrd = bg_texture;
        self.hover = hover_texture;
    }

    pub fn is_inside(&self, x: f32, y: f32) -> bool {
        self.rect.is_inside(x, y)
    }

    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.rect.x, self.rect.y)
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.rect.width, self.rect.height)
    }

    pub fn draw(&self, graphics: &mut Graphics, mouse_pos: Vec2) {
        if self.is_inside(mouse_pos.x, mouse_pos.y) {
            graphics.rect()
                .at(vec2(self.rect.x, self.rect.y))
                .size(vec2(self.rect.width, self.rect.height))
                .texture(self.hover);
        } else {
            graphics.rect()
                .at(vec2(self.rect.x, self.rect.y))
                .size(vec2(self.rect.width, self.rect.height))
                .texture(self.backgrd);
        }
    }

}
