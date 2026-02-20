use crate::engine::ui::{button::Button, rectangle::Rectangle};
use egor::{math::{Vec2, vec2}, render::Graphics};

pub struct ImageButton {
    button: Button,
    image: usize,
    width: f32,
    height: f32,
}

#[allow(dead_code)]
impl ImageButton {

    pub fn new(
        rectangle: Rectangle,
        bg_texture: usize,
        hover_texture: usize,
        image: usize,
        image_width: f32,
        image_height: f32
    ) -> ImageButton {
        let btn = Button::new(rectangle, bg_texture, hover_texture);
        ImageButton {
            button: btn,
            image,
            width: image_width,
            height: image_height
        }
    }

    pub fn is_inside(&self, x: f32, y: f32) -> bool {
        self.button.is_inside(x, y)
    }

    pub fn draw(&self, graphics: &mut Graphics, mouse_pos: Vec2) {
        self.button.draw(graphics, mouse_pos);
        let x = (self.button.get_position().x - self.width) / 2.0;
        let y = (self.button.get_position().y - self.height) / 2.0;
        graphics.rect()
            .texture(self.image)
            .at(vec2(x, y))
            .size(vec2(self.width, self.height));
    }

}
