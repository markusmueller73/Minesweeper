use crate::engine::ui::{button::Button, rectangle::Rectangle};
use egor::{math::{Rect, Vec2}, render::{Align, Color, Graphics}};

pub struct TextButton {
    button: Button,
    text: String,
    font: String,
    color: Color,
}

#[allow(dead_code)]
impl TextButton {

    pub fn new(
        rectangle: Rectangle,
        bg_texture: usize,
        hover_texture: usize,
        text: String,
        font: String,
        color: Color
    ) -> TextButton {
        let btn = Button::new(rectangle, bg_texture, hover_texture);
        TextButton {
            button: btn,
            text,
            font,
            color
        }
    }

    pub fn is_inside(&self, x: f32, y: f32) -> bool {
        self.button.is_inside(x, y)
    }

    pub fn draw(&self, graphics: &mut Graphics, mouse_pos: Vec2) {
        self.button.draw(graphics, mouse_pos);
        graphics.text(self.text.as_str())
            .font(self.font.clone())
            .in_rect(Rect::new(self.button.get_position(), self.button.get_size()), Align::MiddleCenter)
            .color(self.color);
    }

}
